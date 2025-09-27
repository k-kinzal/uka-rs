use crate::tokenizer::Span;
use core::fmt::{Display, Formatter};

/// A token representing a syntactic element in Sakura Script.
///
/// `Token` is an enumeration of all possible token types that can be produced
/// by the Sakura Script lexer. Each token contains the original string content
/// and position information (span) from the source text.
///
/// # Token Types
///
/// - **Text**: Regular text content that doesn't match special syntax
/// - **Escape**: Backslash-prefixed control sequences like `\n`, `\h`, `\_w`
/// - **Percent**: Variable references like `%month`, `%username`, `%screenwidth`
/// - **Bracket**: Square brackets `[` and `]` used for grouping and arguments
/// - **Separator**: Commas `,` used for delimiting elements in lists
/// - **Illegal**: Invalid or incomplete syntax that couldn't be parsed properly
///
/// # Examples
///
/// ## Using tokens from lexer
///
/// ```rust
/// # use uka_sakura_script::tokenizer::{Lexer, Token};
/// let mut lexer = Lexer::new("Hello\\n%username[0],world");
///
/// let token1 = lexer.next_token().unwrap();
/// assert!(matches!(token1, Token::Text(_)));
/// assert_eq!(token1.to_string(), "Hello");
///
/// let token2 = lexer.next_token().unwrap();
/// assert!(matches!(token2, Token::Escape(_)));
/// assert_eq!(token2.to_string(), "\\n");
///
/// let token3 = lexer.next_token().unwrap();
/// assert!(matches!(token3, Token::Percent(_)));
/// assert_eq!(token3.to_string(), "%username");
/// ```
///
/// ## Pattern matching on tokens
///
/// ```rust
/// # use uka_sakura_script::tokenizer::{Lexer, Token};
/// let mut lexer = Lexer::new("Hello\\n");
///
/// while let Some(token) = lexer.next_token() {
///     match token {
///         Token::Text(text) => println!("Text: {}", text),
///         Token::Escape(escape) => println!("Escape: {}", escape),
///         Token::Percent(percent) => println!("Variable: {}", percent),
///         Token::Bracket(bracket) => println!("Bracket: {}", bracket),
///         Token::Separator(sep) => println!("Separator: {}", sep),
///         Token::Illegal(illegal) => println!("Invalid: {}", illegal),
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    Text(Text<'a>),
    Escape(Escape<'a>),
    Percent(Percent<'a>),
    Bracket(Bracket<'a>),
    Separator(Separator<'a>),
    Illegal(Illegal<'a>),
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Token::Text(text) => text.fmt(f),
            Token::Escape(escape) => escape.fmt(f),
            Token::Percent(percent) => percent.fmt(f),
            Token::Bracket(bracket) => bracket.fmt(f),
            Token::Separator(separator) => separator.fmt(f),
            Token::Illegal(illegal) => illegal.fmt(f),
        }
    }
}

/// A token representing regular text content in Sakura Script.
///
/// `Text` tokens contain any characters that are not part of special syntax.
/// This includes regular dialogue text, whitespace, punctuation, and any
/// other content that doesn't trigger special parsing rules.
///
/// # Examples
///
/// ```rust
/// # use uka_sakura_script::tokenizer::{Span, Text};
/// let span = Span::new(0, 5);
/// let text = Text::new("Hello", span);
///
/// assert_eq!(text.to_string(), "Hello");
/// assert_eq!(text.span(), span);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text<'a> {
    str: &'a str,
    span: Span,
}

impl<'a> Text<'a> {
    pub fn new(str: &'a str, span: Span) -> Self {
        Self { str, span }
    }

    /// Returns the span (position information) of this token.
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Display for Text<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.str)
    }
}

/// A token representing an escape sequence in Sakura Script.
///
/// `Escape` tokens represent backslash-prefixed sequences that follow the format `\<character(s)>`.
/// Any character can follow the backslash, making this a flexible mechanism for control sequences,
/// formatting commands, and literal character escapes.
///
/// # Format
/// - Standard format: `\<character>` (reads one character after the backslash)
/// - Special underscore format: `\_<character>` (reads two characters after the backslash)
///
/// The lexer treats underscore (`_`) specially - when it encounters `\_`, it reads the underscore
/// plus one additional character, allowing for extended escape sequences like `\_w`, `\_q`, etc.
///
/// # Examples
///
/// ```rust
/// # use uka_sakura_script::tokenizer::{Span, Escape};
/// let span = Span::new(0, 2);
/// let escape = Escape::new("n", span);
///
/// assert_eq!(escape.to_string(), "\\\\n");
/// assert_eq!(escape.span(), span);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Escape<'a> {
    str: &'a str,
    span: Span,
}

impl<'a> Escape<'a> {
    pub fn new(str: &'a str, span: Span) -> Self {
        assert!(str.len() > 0);

        Self { str, span }
    }

    /// Returns the span (position information) of this token.
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Display for Escape<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(format!("\\{}", self.str).as_str())
    }
}

/// A token representing a percent variable in Sakura Script.
///
/// `Percent` tokens represent variable references that follow the format `%<variable_name>`.
/// Any alphanumeric characters and `?` can form the variable name, making this a flexible
/// mechanism for both built-in system variables and custom user-defined variables.
///
/// # Format and Parsing Behavior
/// - Format: `%<variable_name>` where variable_name contains alphanumeric characters and `?`
/// - The lexer uses shortest-match for recognized built-in patterns
/// - For example, `%msfoo` would be tokenized as `%ms` (not `%msfoo`)
/// - Unknown patterns like `%customvar` or `%foo` are accepted as valid percent tokens
///
/// This design allows for extensibility while providing efficient recognition of common
/// built-in variables through pattern matching in the lexer.
///
/// # Examples
///
/// ```rust
/// # use uka_sakura_script::tokenizer::{Span, Percent};
/// let span = Span::new(0, 6);
/// let percent = Percent::new("month", span);
///
/// assert_eq!(percent.to_string(), "%month");
/// assert_eq!(percent.span(), span);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Percent<'a> {
    str: &'a str,
    span: Span,
}

impl<'a> Percent<'a> {
    pub fn new(str: &'a str, span: Span) -> Self {
        assert!(str.len() > 0);

        Self { str, span }
    }

    /// Returns the span (position information) of this token.
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Display for Percent<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(format!("%{}", self.str).as_str())
    }
}

/// A token representing a square bracket in Sakura Script.
///
/// `Bracket` tokens represent square brackets (`[` and `]`) used for grouping
/// and providing arguments to escape sequences and other commands. They are
/// essential for structuring command arguments and grouping elements.
///
/// # Common Usage
/// - `\h[0]` - Character switching with argument
/// - `\s[10]` - Surface switching with argument  
/// - `\_w[500]` - Wait timing with milliseconds
/// - `[option1,option2]` - Choice options
///
/// # Examples
///
/// ```rust
/// # use uka_sakura_script::tokenizer::{Span, token::Bracket};
/// let span = Span::new(0, 1);
/// let bracket = Bracket::new("[", span);
///
/// assert_eq!(bracket.to_string(), "[");
/// assert_eq!(bracket.span(), span);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bracket<'a> {
    str: &'a str,
    span: Span,
}

impl<'a> Bracket<'a> {
    pub fn new(str: &'a str, span: Span) -> Self {
        assert!(str == "[" || str == "]");

        Self { str, span }
    }

    /// Returns the span (position information) of this token.
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Display for Bracket<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.str)
    }
}

/// A token representing a comma separator in Sakura Script.
///
/// `Separator` tokens represent commas (`,`) used for delimiting elements
/// in lists, particularly within bracket arguments for commands and choices.
/// They provide structure for multi-parameter commands and option lists.
///
/// # Common Usage
/// - `[option1,option2,option3]` - Choice options separated by commas
/// - `[arg1,arg2]` - Multiple arguments to commands
/// - `[x,y,width,height]` - Coordinate and dimension parameters
///
/// # Examples
///
/// ```rust
/// # use uka_sakura_script::tokenizer::{Span, token::Separator};
/// let span = Span::new(0, 1);
/// let separator = Separator::new(",", span);
///
/// assert_eq!(separator.to_string(), ",");
/// assert_eq!(separator.span(), span);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Separator<'a> {
    str: &'a str,
    span: Span,
}

impl<'a> Separator<'a> {
    pub fn new(str: &'a str, span: Span) -> Self {
        assert_eq!(str, ",");

        Self { str, span }
    }

    /// Returns the span (position information) of this token.
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Display for Separator<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.str)
    }
}

/// A token representing invalid or incomplete syntax in Sakura Script.
///
/// `Illegal` tokens are created when the lexer encounters syntax that cannot
/// be properly parsed. This typically occurs with incomplete escape sequences
/// or percent variables that are missing their identifiers.
///
/// # Common Cases
/// - `\` at the end of input without a following character
/// - `%` at the end of input without a variable name
/// - `%` followed by invalid characters that don't form a valid variable name
///
/// These tokens help with error reporting and recovery during parsing,
/// allowing the lexer to continue processing while marking problematic areas.
///
/// # Examples
///
/// ```rust
/// # use uka_sakura_script::tokenizer::{Span, token::Illegal};
/// let span = Span::new(0, 1);
/// let illegal = Illegal::new("\\", span);
///
/// assert_eq!(illegal.to_string(), "\\");
/// assert_eq!(illegal.span(), span);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Illegal<'a> {
    str: &'a str,
    span: Span,
}

impl<'a> Illegal<'a> {
    pub fn new(str: &'a str, span: Span) -> Self {
        assert!(str == "\\" || str == "%");

        Self { str, span }
    }

    /// Returns the span (position information) of this token.
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Display for Illegal<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.str)
    }
}
