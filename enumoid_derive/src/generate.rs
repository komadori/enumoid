//! This module implements the `generate_enumoid!` function-like macro, which
//! declares an enum with a contiguous range of numbered unit variants and a
//! `#[derive(Enumoid)]` attribute.
//!
//! ```ignore
//! generate_enumoid!(Foo, Bar, 1..=3);
//! ```
//!
//! expands to:
//!
//! ```ignore
//! #[derive(Enumoid)]
//! enum Foo {
//!   Bar1,
//!   Bar2,
//!   Bar3,
//! }
//! ```
//!
//! Exclusive ranges (`1..4`) are also supported. Optional leading outer
//! attributes and a visibility may precede the name, e.g.
//! `generate_enumoid!(#[index_type(u16)] pub Foo, Bar, 1..=300);`.

use anyhow::{Result, anyhow};
use syn::parse::{Parse, ParseStream};

/// Parsed form of a `generate_enumoid!` invocation:
/// `[attrs] [vis] Name, VariantPrefix, start..end`.
struct GenerateInput {
  attrs: Vec<syn::Attribute>,
  vis: syn::Visibility,
  name: syn::Ident,
  prefix: syn::Ident,
  start: u64,
  last: u64,
}

/// Parses a single non-negative index bound of the range.
fn parse_index(input: ParseStream) -> syn::Result<u64> {
  if input.peek(syn::Token![-]) {
    return Err(input.error("Negative indices are not supported."));
  }
  input.parse::<syn::LitInt>()?.base10_parse::<u64>()
}

impl Parse for GenerateInput {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let attrs = input.call(syn::Attribute::parse_outer)?;
    let vis = input.parse()?;
    let name = input.parse()?;
    input.parse::<syn::Token![,]>()?;
    let prefix = input.parse()?;
    input.parse::<syn::Token![,]>()?;
    let start = parse_index(input)?;
    // Check `..=` before `..` since the latter is a prefix of the former.
    let inclusive = if input.peek(syn::Token![..=]) {
      input.parse::<syn::Token![..=]>()?;
      true
    } else {
      input.parse::<syn::Token![..]>()?;
      false
    };
    let end = parse_index(input)?;
    let last = if inclusive {
      end
    } else {
      end
        .checked_sub(1)
        .ok_or_else(|| input.error("The range produces no variants."))?
    };
    if start > last {
      return Err(input.error("The range produces no variants."));
    }
    Ok(GenerateInput {
      attrs,
      vis,
      name,
      prefix,
      start,
      last,
    })
  }
}

pub fn try_generate_enumoid(
  input: proc_macro::TokenStream,
) -> Result<proc_macro2::TokenStream> {
  let GenerateInput {
    attrs,
    vis,
    name,
    prefix,
    start,
    last,
  } = syn::parse(input).map_err(|e| anyhow!("{}", e))?;
  let variants = (start..=last)
    .map(|i| format_ident!("{}{}", prefix, i))
    .collect::<Vec<_>>();
  // `#[derive(Enumoid)]` must precede the caller's attributes so that its
  // helper attributes (e.g. `#[index_type]`) are introduced before their use.
  Ok(quote! {
    #[derive(Enumoid)]
    #(#attrs)*
    #vis enum #name {
      #(#variants),*
    }
  })
}
