mod cursor;
mod lexer;
mod span;
mod stream;
mod token;

pub use cursor::Cursor;
pub use lexer::Lexer;
pub use span::Span;
pub use stream::TokenStream;
pub use token::{Bracket, Escape, Illegal, Percent, Separator, Text, Token};

/// Creates a token stream from an input string for Sakura Script tokenization.
///
/// This is a convenience function that creates a `TokenStream` from the provided
/// input string. It's equivalent to calling `TokenStream::from(input)` but provides
/// a more explicit function-based API for tokenization.
///
/// The function parses the input according to Sakura Script syntax rules,
/// recognizing text content, escape sequences, percent variables, brackets,
/// separators, and illegal tokens.
///
/// # Examples
///
/// ```rust
/// # use uka_sakura_script::tokenizer::tokenize;
/// let stream = tokenize("Hello\\\\n[0]%month");
/// let tokens: Vec<_> = stream.collect();
///
/// assert_eq!(tokens.len(), 7);
/// assert_eq!(tokens[0].to_string(), "Hello");
/// assert_eq!(tokens[1].to_string(), "\\\\");
/// assert_eq!(tokens[2].to_string(), "n");
/// assert_eq!(tokens[3].to_string(), "[");
/// assert_eq!(tokens[4].to_string(), "0");
/// assert_eq!(tokens[5].to_string(), "]");
/// assert_eq!(tokens[6].to_string(), "%month");
pub fn tokenize(input: &str) -> TokenStream {
    TokenStream::from(input)
}
