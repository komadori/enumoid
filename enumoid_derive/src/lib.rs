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

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

/// Derive macro which implements the `Enumoid` trait
#[proc_macro_derive(Enumoid)]
pub fn derive_enumoid(
  input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  let input: syn::DeriveInput = syn::parse(input).unwrap();
  let output = if let syn::Data::Enum(data_enum) = input.data {
    let name = input.ident;
    let elem_count = data_enum.variants.len();
    let flag_bytes = (elem_count + 7) / 8;
    let word_type = match elem_count {
      0..=0xff => quote! { u8 },
      0x100..=0xffff => quote! { u16 },
      0x10000..=0xffffffff => quote! { u32 },
      _ => quote! { usize },
    };
    let elem_count_lit = proc_macro2::Literal::usize_unsuffixed(elem_count);
    let variant_names: Vec<&proc_macro2::Ident> =
      data_enum.variants.iter().map(|x| &x.ident).collect();
    let indices: Vec<_> = (0..elem_count)
      .map(proc_macro2::Literal::usize_unsuffixed)
      .collect();
    let bounded = if variant_names.is_empty() {
      quote! {}
    } else {
      let first_variant = variant_names.first().unwrap();
      let last_variant = variant_names.last().unwrap();
      quote! {
        impl enumoid::Enumoid1 for #name {
          const FIRST: Self = #name::#first_variant;
          const LAST: Self = #name::#last_variant;
        }
      }
    };
    quote! {
      impl enumoid::Enumoid for #name {
        type Word = #word_type;
        type WordRange = std::ops::Range<#word_type>;
        type FlagsArray = [u8; #flag_bytes];
        const SIZE: usize = #elem_count_lit;
        const SIZE_WORD: #word_type = #elem_count_lit;
        const ZERO_WORD: #word_type = 0;
        const ONE_WORD: #word_type = 1;
        const DEFAULT_FLAGS: Self::FlagsArray = [0; #flag_bytes];
        const FLAGS_BITS: usize = 8;
        const FLAGS_BITS_WORD: #word_type = 8;
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
      #bounded
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
    }
  } else {
    quote!(compile_error! {"#[derive(Enumoid)] can only be applied to enums"})
  };
  output.into()
}
