mod types;

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, LitByteStr, LitInt, Token};
use types::ByteLiteral;

/// BytesSliceInput is a struct for parsing input of bytes_slice! and bytes_slice_length! macro.
struct BytesSliceInput {
    bytes: LitByteStr,
    start: LitInt,
    end: Option<LitInt>,
}

impl Parse for BytesSliceInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let bytes = input.parse()?;
        input.parse::<Token![,]>()?;
        let start = input.parse()?;
        if input.parse::<Token![,]>().is_ok() {
            let end = input.parse()?;
            Ok(Self {
                bytes,
                start,
                end: Some(end),
            })
        } else {
            Ok(Self {
                bytes,
                start,
                end: None,
            })
        }
    }
}

/// bytes_slices! is a macro that slices byte literals.
///
/// # Example
///
/// ```rust
/// # use bytes_macro::bytes_slice;
/// #
/// assert_eq!(bytes_slice!(b"hello", 1), b"ello");
/// assert_eq!(bytes_slice!(b"hello", 1, 4), b"ell");
/// ```
#[proc_macro]
pub fn bytes_slice(input: TokenStream) -> TokenStream {
    let BytesSliceInput { bytes, start, end } = parse_macro_input!(input as BytesSliceInput);
    let start = start.base10_digits().parse().unwrap();
    let bytes = if let Some(end) = end {
        let end = end.base10_digits().parse().unwrap();
        bytes.value()[start..end].to_vec()
    } else {
        bytes.value()[start..].to_vec()
    };
    let lit = ByteLiteral::from(bytes);

    quote!(#lit).into()
}

/// bytes_slice_length! is a macro that returns the length of sliced byte literals.
///
/// # Example
///
/// ```rust
/// # use bytes_macro::bytes_slice_length;
/// #
/// assert_eq!(bytes_slice_length!(b"hello", 1), 4);
/// assert_eq!(bytes_slice_length!(b"hello", 1, 4), 3);
/// ```
#[proc_macro]
pub fn bytes_slice_length(input: TokenStream) -> TokenStream {
    let BytesSliceInput { bytes, start, end } = parse_macro_input!(input as BytesSliceInput);
    let start = start.base10_digits().parse().unwrap();
    let bytes = if let Some(end) = end {
        let end = end.base10_digits().parse().unwrap();
        bytes.value()[start..end].to_vec()
    } else {
        bytes.value()[start..].to_vec()
    };
    let len = bytes.len();

    quote!(#len).into()
}

/// bytes_length! is a macro that returns the length of byte literals.
///
/// # Example
///
/// ```rust
/// # use bytes_macro::bytes_length;
/// #
/// assert_eq!(bytes_length!(b"hello"), 5);
/// ```
#[proc_macro]
pub fn bytes_length(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitByteStr);
    let len = input.value().len();

    quote!(#len).into()
}
