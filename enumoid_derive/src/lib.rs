//! This crate provides Enumoid's derive macro.
//!
//! ```
//! # use enumoid_derive::Enumoid;
//! #
//! #[derive(Enumoid)]
//! # enum E { One }
//! #
//! # fn main() {}
//! ```

use anyhow::{anyhow, bail, Result};
use quote::ToTokens;

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

fn get_index_type(
  input: &syn::DeriveInput,
) -> Result<proc_macro2::TokenStream> {
  for attr in input.attrs.iter() {
    if attr.path().is_ident("index_type") {
      return match attr.parse_args() {
        Ok(syn::TypePath { path, .. }) if path.is_ident("u8") => {
          Ok(quote! { u8 })
        }
        Ok(syn::TypePath { path, .. }) if path.is_ident("u16") => {
          Ok(quote! { u16 })
        }
        Ok(syn::TypePath { path, .. }) if path.is_ident("u32") => {
          Ok(quote! { u32 })
        }
        Ok(syn::TypePath { path, .. }) if path.is_ident("usize") => {
          Ok(quote! { usize })
        }
        Ok(e) => {
          bail!(
            "Invalid argument to index_type attribute: {}",
            e.into_token_stream()
          );
        }
        Err(e) => {
          bail!("Error parsing arguments to index_type attribute: {}", e);
        }
      };
    }
  }
  Ok(quote! { u8 })
}

fn get_bitset_word_types(
  input: &syn::DeriveInput,
) -> Result<Vec<proc_macro2::TokenStream>> {
  let mut tys = vec![quote! {u8}, quote! {usize}];
  for attr in input.attrs.iter() {
    if attr.path().is_ident("bitset_word_types") {
      let ty_list = attr.parse_args_with(syn::punctuated::Punctuated::<syn::TypePath, syn::Token![,]>::parse_terminated)?;
      tys.clear();
      for ty in ty_list {
        tys.push(ty.into_token_stream())
      }
    }
  }
  Ok(tys)
}
struct Rule {
  size: proc_macro2::TokenStream,
  consts: proc_macro2::TokenStream,
  to_expr: proc_macro2::TokenStream,
  from_expr: proc_macro2::TokenStream,
  first: proc_macro2::TokenStream,
  last: proc_macro2::TokenStream,
}

fn generate_enum_rules(
  data: &syn::DataEnum,
  name: &syn::Ident,
) -> Result<Vec<Rule>> {
  let mut next = quote! { 0 };
  let mut rules = Vec::new();
  for (index, variant) in data.variants.iter().enumerate() {
    let v_name = &variant.ident;
    let kns = format_ident!("K{}S", index.to_string());
    rules.push(if let Some(field) = variant.fields.iter().next() {
      if variant.fields.len() > 1 {
        bail!("Enumoid variants may not have more than one field.");
      }
      if field.ident.is_some() {
        bail!("Enumoid variants may not use a named field.")
      }
      let kne = format_ident!("K{}E", index.to_string());
      let sub_ty = field.ty.clone();
      let curr = next;
      next = quote! { #kns + <#sub_ty as enumoid::Enumoid>::SIZE_WORD as <#name as enumoid::Enumoid>::Word };
      Rule {
        size: quote! { <#sub_ty as enumoid::Enumoid>::SIZE },
        consts: quote! {
          const #kns: <#name as enumoid::Enumoid>::Word = #curr;
          const #kne: <#name as enumoid::Enumoid>::Word = #next - 1;
        },
        to_expr: quote! { Self::#v_name(x) => #kns + x.into_word() as <#name as enumoid::Enumoid>::Word, },
        from_expr: quote! { x@#kns..=#kne => Self::#v_name(#sub_ty::from_word_unchecked((x-#kns) as <#sub_ty as enumoid::Enumoid>::Word)), },
        first: quote! { Self::#v_name(#sub_ty::FIRST) },
        last: quote! { Self::#v_name(#sub_ty::LAST) },
      }
    } else {
      let rule = Rule {
        size: quote! { 1 },
        consts: quote! { const #kns: <#name as enumoid::Enumoid>::Word = #next; },
        to_expr: quote! { Self::#v_name => #kns, },
        from_expr: quote! { #kns => Self::#v_name, },
        first: quote! { Self::#v_name },
        last: quote! { Self::#v_name },
      };
      next = quote! { #kns + 1 };
      rule
    });
  }
  Ok(rules)
}

fn generate_struct_rules(
  data: &syn::DataStruct,
  name: &syn::Ident,
) -> Result<Vec<Rule>> {
  let rule = if let Some(field) = data.fields.iter().next() {
    if data.fields.len() > 1 {
      bail!("Enumoid structs may not have more than one field.");
    }
    if field.ident.is_some() {
      bail!("Enumoid structs may not use a named field.")
    }
    let sub_ty = field.ty.clone();
    Rule {
      size: quote! { <#sub_ty as enumoid::Enumoid>::SIZE },
      consts: quote! {},
      to_expr: quote! { #name(x) => x.into_word() as <#name as enumoid::Enumoid>::Word, },
      from_expr: quote! { x => #name(#sub_ty::from_word_unchecked((x) as <#sub_ty as enumoid::Enumoid>::Word)), },
      first: quote! { #name(<#sub_ty as enumoid::Enumoid>::FIRST) },
      last: quote! { #name(<#sub_ty as enumoid::Enumoid>::LAST) },
    }
  } else {
    Rule {
      size: quote! { 1 },
      consts: quote! {},
      to_expr: quote! { #name => 0, },
      from_expr: quote! { 0 => #name, },
      first: quote! { #name },
      last: quote! { #name },
    }
  };
  Ok(vec![rule])
}

fn try_derive_enumoid(
  input: proc_macro::TokenStream,
) -> Result<proc_macro2::TokenStream> {
  let input: syn::DeriveInput = syn::parse(input).unwrap();
  let word_type = get_index_type(&input)?;
  let word_type_error = format!("Index type '{}' is too narrow.", word_type);
  let bitset_word_types = get_bitset_word_types(&input)?;
  let name = input.ident;
  let rules = if let syn::Data::Enum(data_enum) = input.data {
    if data_enum.variants.is_empty() {
      Err(anyhow!("Enumoids must be inhabited by at least one value."))
    } else {
      generate_enum_rules(&data_enum, &name)
    }
  } else if let syn::Data::Struct(data_struct) = input.data {
    generate_struct_rules(&data_struct, &name)
  } else {
    Err(anyhow!(
      "#[derive(Enumoid)] must be applied to an enum or struct."
    ))
  }?;
  let size = rules
    .iter()
    .map(|r| r.size.clone())
    .reduce(|a, b| quote! { #a + #b });
  let consts: Vec<&proc_macro2::TokenStream> =
    rules.iter().map(|r| &r.consts).collect();
  let to_exprs = rules.iter().map(|r| &r.to_expr);
  let from_exprs = rules.iter().map(|r| &r.from_expr);
  let first = &rules.first().unwrap().first;
  let last = &rules.last().unwrap().last;
  Ok(quote! {
    impl enumoid::Enumoid for #name {
      type Word = #word_type;
      type WordRange = std::ops::Range<Self::Word>;
      const SIZE: usize = #size;
      const SIZE_WORD: Self::Word = if Self::SIZE <= #word_type::MAX as usize {
        Self::SIZE as Self::Word
      }
      else
      {
        panic!(#word_type_error);
      };
      const FIRST: Self = #first;
      const LAST: Self = #last;
      #[inline]
      fn into_word(self) -> Self::Word {
        #(
          #consts
        )*
        match self {
          #(
            #to_exprs
          )*
        }
      }
      #[inline]
      unsafe fn from_word_unchecked(value: Self::Word) -> Self {
        debug_assert!(
          value < Self::SIZE_WORD,
          "from_word_unchecked: Index out of bounds: {:?} >= {:?}",
          value,
          Self::SIZE_WORD
        );
        #(
          #consts
        )*
        match value {
          #(
            #from_exprs
          )*
          _ => unsafe { std::hint::unreachable_unchecked() }
        }
      }
      #[inline]
      fn word_range(base: Self::Word, lim: Self::Word) -> Self::WordRange {
        base..lim
      }
    }
    impl<V> enumoid::EnumArrayHelper<V> for #name {
      type PartialArray = [std::mem::MaybeUninit<V>; <Self as enumoid::Enumoid>::SIZE];
      type TotalArray = [V; <Self as enumoid::Enumoid>::SIZE];
      #[inline(always)]
      fn partial_slice(p: &Self::PartialArray)
        -> &[std::mem::MaybeUninit<V>] { p }
      #[inline(always)]
      fn partial_slice_mut(p: &mut Self::PartialArray)
        -> &mut [std::mem::MaybeUninit<V>] { p }
      #[inline]
      unsafe fn partial_to_total(p: Self::PartialArray)
        -> Self::TotalArray {
        std::ptr::read(&p as *const _ as *const Self::TotalArray)
      }
      #[inline(always)]
      fn total_slice(t: &Self::TotalArray) -> &[V] { t }
      #[inline(always)]
      fn total_slice_mut(t: &mut Self::TotalArray) -> &mut [V] { t }
      #[inline]
      fn total_to_partial(t: Self::TotalArray)
        -> Self::PartialArray {
        let p = unsafe {
          std::ptr::read(&t as *const _ as *const Self::PartialArray)
        };
        std::mem::forget(t);
        p
      }
    }
    #(
      impl enumoid::EnumSetHelper<#bitset_word_types> for #name {
        type BitsetWord = #bitset_word_types;
        type BitsetArray = [#bitset_word_types; <Self as enumoid::EnumSetHelper<#bitset_word_types>>::BITSET_WORDS];
        const BITSET_WORD_BITS: usize = <#bitset_word_types>::BITS as usize;
        const DEFAULT_BITSET: Self::BitsetArray = [0; <Self as enumoid::EnumSetHelper<#bitset_word_types>>::BITSET_WORDS];
        #[inline(always)]
        fn slice_bitset(arr: &Self::BitsetArray) -> &[#bitset_word_types] { arr }
        #[inline(always)]
        fn slice_bitset_mut(arr: &mut Self::BitsetArray) -> &mut [#bitset_word_types] { arr }
      }
    )*
    impl core::convert::From<enumoid::EnumIndex<#name>> for #name {
      #[inline]
      fn from(index: enumoid::EnumIndex<#name>) -> Self {
        index.into_value()
      }
    }
  })
}

/// Derive macro which implements the `Enumoid`, `EnumArrayHelper<V>`,
/// `EnumSetHelper<BitsetWord>`, and `From<EnumIndex<T>>` traits for
/// a type.
#[proc_macro_derive(Enumoid, attributes(index_type, bitset_word_types))]
pub fn derive_enumoid(
  input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  match try_derive_enumoid(input) {
    Ok(q) => q,
    Err(e) => {
      let msg = e.to_string();
      quote! { compile_error!(#msg); }
    }
  }
  .into()
}
