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

use anyhow::{bail, Result};
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

fn try_derive_enumoid(
  input: proc_macro::TokenStream,
) -> Result<proc_macro2::TokenStream> {
  let input: syn::DeriveInput = syn::parse(input).unwrap();
  let word_type = get_index_type(&input)?;
  if let syn::Data::Enum(data_enum) = input.data {
    let name = input.ident;
    let elem_count = data_enum.variants.len();
    if elem_count == 0 {
      bail!("Enumoids must be inhabited by at least one value.");
    }
    let word_type_error = format!(
      "Index type '{}' is too narrow for {} values.",
      word_type, elem_count
    );
    let elem_count_lit = proc_macro2::Literal::usize_unsuffixed(elem_count);
    let variant_names: Vec<&proc_macro2::Ident> =
      data_enum.variants.iter().map(|x| &x.ident).collect();
    let indices: Vec<_> = (0..elem_count)
      .map(proc_macro2::Literal::usize_unsuffixed)
      .collect();
    let first_variant = variant_names.first().unwrap();
    let last_variant = variant_names.last().unwrap();
    Ok(quote! {
      impl enumoid::Enumoid for #name {
        type Word = #word_type;
        type WordRange = std::ops::Range<#word_type>;
        type FlagsArray = [u8; Self::FLAGS_WORDS];
        const SIZE: usize = #elem_count_lit;
        const SIZE_WORD: #word_type = if Self::SIZE <= #word_type::MAX as usize {
          Self::SIZE as #word_type
        }
        else
        {
          panic!(#word_type_error);
        };
        const FIRST: Self = #name::#first_variant;
        const LAST: Self = #name::#last_variant;
        const FLAGS_BITS: usize = 8;
        const DEFAULT_FLAGS: Self::FlagsArray = [0; Self::FLAGS_WORDS];
        #[inline]
        fn into_word(self) -> Self::Word {
          match self {
            #(
              #name::#variant_names => #indices,
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
          match value {
            #(
              #indices => #name::#variant_names,
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
        type PartialArray = [std::mem::MaybeUninit<V>; #elem_count];
        type TotalArray = [V; #elem_count];
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
  } else {
    bail!("#[derive(Enumoid)] can only be applied to enums");
  }
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
