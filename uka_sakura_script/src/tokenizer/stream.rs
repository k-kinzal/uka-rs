use crate::tokenizer::{Lexer, Token};

/// A token stream iterator for Sakura Script syntax.
///
/// `TokenStream` provides a convenient iterator interface for tokenizing Sakura Script
/// input text. It wraps a `Lexer` and implements the `Iterator` trait, allowing for
/// easy sequential processing of tokens using standard Rust iterator methods.
///
/// This stream-based approach is ideal for parsing workflows where you need to
/// process tokens one by one or apply iterator transformations like filtering,
/// mapping, or collecting tokens into collections.
///
/// # Examples
///
/// ```rust
/// # use uka_sakura_script::tokenizer::{TokenStream, Token};
/// let stream = TokenStream::from("Hello\\n[0]%month");
///
/// for token in stream {
///     println!("Token: {}", token);
/// }
/// // Outputs:
/// // Token: Hello
/// // Token: \n
/// // Token: [
/// // Token: 0
/// // Token: ]
/// // Token: %month
/// ```
pub struct TokenStream<'a> {
    inner: Lexer<'a>,
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next_token()
    }
}

impl<'a> From<&'a str> for TokenStream<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            inner: Lexer::new(value),
        }
    }
}
