//! This crate provides Enumoid's derive macro.
//!
//! ```
//! # use enumoid_derive::Enumoid;
//! #
//! #[derive(Enumoid)]
//! # enum E {}
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
      next = quote! { #kns + <#sub_ty as Enumoid>::SIZE_WORD as <#name as Enumoid>::Word };
      Rule {
        size: quote! { <#sub_ty as Enumoid>::SIZE },
        consts: quote! {
          const #kns: <#name as Enumoid>::Word = #curr;
          const #kne: <#name as Enumoid>::Word = #next - 1;
        },
        to_expr: quote! { Self::#v_name(x) => #kns + x.into_word() as <#name as Enumoid>::Word, },
        from_expr: quote! { x@#kns..=#kne => Self::#v_name(#sub_ty::from_word_unchecked((x-#kns) as <#sub_ty as Enumoid>::Word)), },
        first: quote! { Self::#v_name(#sub_ty::FIRST) },
        last: quote! { Self::#v_name(#sub_ty::LAST) },
      }
    } else {
      let rule = Rule {
        size: quote! { 1 },
        consts: quote! { const #kns: <#name as Enumoid>::Word = #next; },
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

fn try_derive_enumoid(
  input: proc_macro::TokenStream,
) -> Result<proc_macro2::TokenStream> {
  let input: syn::DeriveInput = syn::parse(input).unwrap();
  let word_type = get_index_type(&input)?;
  let word_type_error = format!("Index type '{}' is too narrow.", word_type);
  let name = input.ident;
  let rules = if let syn::Data::Enum(data_enum) = input.data {
    if data_enum.variants.is_empty() {
      Err(anyhow!("Enumoids must be inhabited by at least one value."))
    } else {
      generate_enum_rules(&data_enum, &name)
    }
  } else {
    Err(anyhow!("#[derive(Enumoid)] can only be applied to enums."))
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
      type FlagsArray = [u8; Self::FLAGS_WORDS];
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
      const FLAGS_BITS: usize = 8;
      const DEFAULT_FLAGS: Self::FlagsArray = [0; Self::FLAGS_WORDS];
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
      #[inline(always)]
      fn slice_flags(arr: &Self::FlagsArray) -> &[u8] { arr }
      #[inline(always)]
      fn slice_flags_mut(arr: &mut Self::FlagsArray) -> &mut [u8] { arr }
    }
    impl<V> enumoid::EnumArrayHelper<V> for #name {
      type PartialArray = [std::mem::MaybeUninit<V>; Self::SIZE];
      type TotalArray = [V; Self::SIZE];
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
  })
}

/// Derive macro which implements the `Enumoid` trait
#[proc_macro_derive(Enumoid, attributes(index_type))]
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
