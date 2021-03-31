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

#[proc_macro_derive(Enumoid)]
pub fn derive_enumoid(
  input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  let input: syn::DeriveInput = syn::parse(input).unwrap();
  let output = if let syn::Data::Enum(data_enum) = input.data {
    let name = input.ident;
    let elem_count = data_enum.variants.len();
    let flag_bytes = (elem_count + 7) / 8;
    let sz_type = match elem_count {
      0..=0xff => quote! { u8 },
      0x100..=0xffff => quote! { u16 },
      0x10000..=0xffffffff => quote! { u32 },
      _ => quote! { usize },
    };
    let variant_names: Vec<&proc_macro2::Ident> =
      data_enum.variants.iter().map(|x| &x.ident).collect();
    //let last_variant = variant_names.last();
    let indices: Vec<_> = (0..elem_count).collect();
    quote! {
      impl enumoid::Enumoid for #name {
        type CompactSize = #sz_type;
        const SIZE: usize = #elem_count;
        //const LAST: Self = #name::#last_variant;
        #[inline]
        fn into_usize(value: Self) -> usize {
          match value {
            #(
              #name::#variant_names => #indices,
            )*
          }
        }
        #[inline]
        fn from_usize(value: usize) -> Self {
          match value {
            #(
              #indices => #name::#variant_names,
            )*
            _ => unreachable!()
          }
        }
        #[inline(always)]
        fn uncompact_size(sz: Self::CompactSize) -> usize
        {
          sz as usize
        }
        #[inline(always)]
        fn compact_size(sz: usize) -> Self::CompactSize
        {
          sz as Self::CompactSize
        }
      }
      impl enumoid::base::EnumFlagsHelper for #name {
        type FlagsArray = [u8; #flag_bytes];
        #[inline(always)]
        fn slice_flags(arr: &Self::FlagsArray) -> &[u8] { arr }
        #[inline(always)]
        fn slice_flags_mut(arr: &mut Self::FlagsArray) -> &mut [u8] { arr }
      }
      impl<V> enumoid::base::EnumArrayHelper<V> for #name {
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
          enumoid::base::unconstrained_transmute::<_, Self::TotalArray>(p) }
        #[inline(always)]
        fn total_slice(t: &Self::TotalArray) -> &[V] { t }
        #[inline(always)]
        fn total_slice_mut(t: &mut Self::TotalArray) -> &mut [V] { t }
        #[inline]
        fn total_to_partial(t: Self::TotalArray)
          -> Self::PartialArray {
          unsafe {
            enumoid::base::unconstrained_transmute::<_, Self::PartialArray>(
              t)
          }
        }
      }
    }
  } else {
    quote!(compile_error! {"#[derive(Enumoid)] can only be applied to enums"})
  };
  output.into()
}
