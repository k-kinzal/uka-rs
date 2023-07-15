use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::LitByteStr;

/// ByteLiteral is a struct for converting Vec<u8> to TokenStream.
pub struct ByteLiteral(Vec<u8>);

impl ToTokens for ByteLiteral {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let bytes = &self.0;
        let bytes = LitByteStr::new(bytes.as_slice(), Span::call_site());
        tokens.extend(quote! {
            #bytes
        });
    }
}

impl From<Vec<u8>> for ByteLiteral {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}
