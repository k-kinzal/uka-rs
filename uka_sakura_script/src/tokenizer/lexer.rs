use crate::tokenizer::{Bracket, Cursor, Escape, Illegal, Percent, Separator, Text};
use crate::tokenizer::{Span, Token};

/// A lexical analyzer for tokenizing Sakura Script syntax.
///
/// `Lexer` provides a token-based parsing interface for Sakura Script, a specialized
/// scripting language used in ghost software. It breaks down input text into meaningful
/// tokens such as text content, escape sequences, percent variables, brackets, and separators
/// while maintaining proper position tracking through spans.
///
/// The lexer recognizes several token types:
/// - **Text**: Regular text content that doesn't match special syntax
/// - **Escape**: Backslash-prefixed sequences like `\n`, `\t`, `\_w`
/// - **Percent**: Variable references like `%month`, `%selfname`, `%screenwidth`
/// - **Bracket**: Square brackets `[` and `]` used for grouping
/// - **Separator**: Commas `,` used for delimiting elements
/// - **Illegal**: Invalid or incomplete syntax that couldn't be parsed
///
/// # Examples
///
/// ## Basic tokenization
///
/// ```rust
/// # use uka_sakura_script::tokenizer::Lexer;
/// let mut lexer = Lexer::new("Hello\\n[0]");
///
/// // First token: "Hello" (Text)
/// let token1 = lexer.next_token().unwrap();
/// assert_eq!(token1.to_string(), "Hello");
///
/// // Second token: "\\n" (Escape)
/// let token2 = lexer.next_token().unwrap();
/// assert_eq!(token2.to_string(), "\\n");
///
/// // Third token: "[" (Bracket)
/// let token3 = lexer.next_token().unwrap();
/// assert_eq!(token3.to_string(), "[");
/// ```
///
/// ## Processing percent variables
///
/// ```rust
/// # use uka_sakura_script::tokenizer::Lexer;
/// let mut lexer = Lexer::new("%month%day");
///
/// // Tokenizes built-in variables
/// let month = lexer.next_token().unwrap();
/// assert_eq!(month.to_string(), "%month");
///
/// let day = lexer.next_token().unwrap();
/// assert_eq!(day.to_string(), "%day");
/// ```
///
/// ## Handling escape sequences
///
/// ```rust
/// # use uka_sakura_script::tokenizer::Lexer;
/// let mut lexer = Lexer::new("\\_w[500]\\n");
///
/// // Special escape for wait timing
/// let wait = lexer.next_token().unwrap();
/// assert_eq!(wait.to_string(), "\\_w");
///
/// // Bracket with timing value
/// let bracket = lexer.next_token().unwrap();
/// assert_eq!(bracket.to_string(), "[");
/// ```
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            cursor: Cursor::new(input),
        }
    }

    /// Reads and returns the next token from the input stream.
    pub fn next_token(&mut self) -> Option<Token<'a>> {
        self.cursor.peek().and_then(|c| match c {
            '\\' => self.next_token_escape(),
            '%' => self.next_token_percent(),
            '[' | ']' => Some(Token::Bracket(Bracket::new(
                self.cursor.read(1),
                Span::new(self.cursor.position(), self.cursor.position() + 1),
            ))),
            ',' => Some(Token::Separator(Separator::new(
                self.cursor.read(1),
                Span::new(self.cursor.position(), self.cursor.position() + 1),
            ))),
            _ => self.next_token_text(),
        })
    }

    fn next_token_text(&mut self) -> Option<Token<'a>> {
        let start_pos = self.cursor.position();
        let s = self.cursor.read_while(|c| !"\\%[],".contains(c));
        if s.is_empty() {
            None
        } else {
            Some(Token::Text(Text::new(
                s,
                Span::new(start_pos, self.cursor.position()),
            )))
        }
    }

    /// Reads an escape sequence token starting with backslash.
    ///
    /// Escape sequences in Sakura Script follow the format `\\<character(s)>` where
    /// any character can follow the backslash. This provides a flexible mechanism
    /// for control sequences, formatting commands, and literal character escapes.
    ///
    /// ## Parsing Rules:
    /// - Standard format: `\\<character>` (reads one character after backslash)
    /// - Special underscore format: `\\_<character>` (reads two characters after backslash)
    /// - Examples: `\\n`, `\\h`, `\\z`, `\\_w`, `\\_q`, `\\\\`, `\\%`, `\\foo`, etc.
    ///
    /// The underscore (`_`) is treated specially - when encountered after a backslash,
    /// the lexer reads both the underscore and the following character as a single escape.
    /// A backslash at the end of input creates an illegal token.
    fn next_token_escape(&mut self) -> Option<Token<'a>> {
        let start_pos = self.cursor.position();
        assert_eq!(self.cursor.next(), Some('\\'));

        match self.cursor.peek() {
            Some('_') => {
                let content = self.cursor.read(2);
                Some(Token::Escape(Escape::new(
                    content,
                    Span::new(start_pos, self.cursor.position()),
                )))
            }
            Some(_) => {
                let content = self.cursor.read(1);
                Some(Token::Escape(Escape::new(
                    content,
                    Span::new(start_pos, self.cursor.position()),
                )))
            }
            None => Some(Token::Illegal(Illegal::new(
                "\\",
                Span::new(start_pos, self.cursor.position()),
            ))),
        }
    }

    /// Reads a percent variable token.
    ///
    /// Percent variables in Sakura Script follow the format `%<variable_name>` where
    /// variable_name can contain alphanumeric characters and question marks. This method
    /// uses shortest-match pattern recognition for efficiency with built-in variables,
    /// but accepts any valid variable name format.
    ///
    /// ## Parsing Strategy:
    /// - First attempts to match known built-in variable patterns using shortest-match
    /// - Examples of built-in patterns: `month`, `day`, `hour`, `username`, `ms`, etc.
    /// - If `%msfoo` is encountered, it matches as `%ms` (shortest-match behavior)
    /// - For unrecognized patterns, reads any alphanumeric characters and `?`
    /// - Custom variables like `%foo`, `%customvar` are valid and accepted
    ///
    /// A standalone `%` with no valid identifier creates an illegal token.
    fn next_token_percent(&mut self) -> Option<Token<'a>> {
        assert_eq!(self.cursor.next(), Some('%'));

        let s = match self.cursor.peek_nth(2) {
            "mo" => self.cursor.read(6), // %month
            "da" => self.cursor.read(4), // %day
            "ho" => self.cursor.read(5), // %hour
            "mi" => self.cursor.read(7), // %minute
            "us" => self.cursor.read(9), // %username
            "se" => match self.cursor.peek_nth(6) {
                "second" => self.cursor.read(6), // %second
                "selfna" => match self.cursor.peek_nth(9) {
                    "selfname2" => self.cursor.read(9), // %selfname2
                    _ => self.cursor.read(8),           // %selfname
                },
                _ => "",
            },
            "ke" => self.cursor.read(10), // %keroname
            "sc" => match self.cursor.peek_nth(7) {
                "screenw" => self.cursor.read(11), // %screenwidth
                "screenh" => self.cursor.read(12), // %screenheight
                _ => "",
            },
            "ex" => self.cursor.read(4), // %exh
            "so" => self.cursor.read(8), // %songname
            "ms" => self.cursor.read(3), // %ms
            "mz" => self.cursor.read(3), // %mz
            "ml" => self.cursor.read(3), // %ml
            "mc" => self.cursor.read(3), // %mc
            "mh" => self.cursor.read(3), // %mh
            "mt" => self.cursor.read(3), // %mt
            "me" => self.cursor.read(3), // %me
            "mp" => self.cursor.read(3), // %mp
            "m?" => self.cursor.read(3), // %m?
            "dm" => self.cursor.read(4), // %dms
            "j[" => self.cursor.read(2), // %j
            _ => "",
        };
        if s.is_empty() {
            let ss = self.cursor.read_while(|c| {
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789?".contains(c)
            });
            if ss.is_empty() {
                Some(Token::Illegal(Illegal::new(
                    "%",
                    Span::new(self.cursor.position() - 1, self.cursor.position()),
                )))
            } else {
                Some(Token::Percent(Percent::new(
                    ss,
                    Span::new(
                        self.cursor.position() - ss.len() - 1,
                        self.cursor.position(),
                    ),
                )))
            }
        } else {
            Some(Token::Percent(Percent::new(
                s,
                Span::new(self.cursor.position() - s.len() - 1, self.cursor.position()),
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::Token;

    #[test]
    fn test_next_token_text() {
        let mut lexer = Lexer::new("こんにちは");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Text(_)));
        assert_eq!(token.to_string(), "こんにちは");
    }

    #[test]
    fn test_next_token_escape() {
        let mut lexer = Lexer::new("\\n");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Escape(_)));
        assert_eq!(token.to_string(), "\\n");
    }

    #[test]
    fn test_next_token_percent() {
        let mut lexer = Lexer::new("%month");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Percent(_)));
        assert_eq!(token.to_string(), "%month");
    }

    #[test]
    fn test_next_token_bracket_left() {
        let mut lexer = Lexer::new("[");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Bracket(_)));
        assert_eq!(token.to_string(), "[");
    }

    #[test]
    fn test_next_token_bracket_right() {
        let mut lexer = Lexer::new("]");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Bracket(_)));
        assert_eq!(token.to_string(), "]");
    }

    #[test]
    fn test_next_token_separator() {
        let mut lexer = Lexer::new(",");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Separator(_)));
        assert_eq!(token.to_string(), ",");
    }

    #[test]
    fn test_next_token_empty_input() {
        let mut lexer = Lexer::new("");
        assert!(lexer.next_token().is_none());
    }

    #[test]
    fn test_next_token_escape_with_underscore() {
        let mut lexer = Lexer::new("\\_w");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Escape(_)));
        assert_eq!(token.to_string(), "\\_w");
    }

    #[test]
    fn test_next_token_escape_backslash() {
        let mut lexer = Lexer::new("\\\\");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Escape(_)));
        assert_eq!(token.to_string(), "\\\\");
    }

    #[test]
    fn test_next_token_escape_percent() {
        let mut lexer = Lexer::new("\\%");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Escape(_)));
        assert_eq!(token.to_string(), "\\%");
    }

    #[test]
    fn test_next_token_percent_selfname2() {
        let mut lexer = Lexer::new("%selfname2");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Percent(_)));
        assert_eq!(token.to_string(), "%selfname2");
    }

    #[test]
    fn test_next_token_percent_screenwidth() {
        let mut lexer = Lexer::new("%screenwidth");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Percent(_)));
        assert_eq!(token.to_string(), "%screenwidth");
    }

    #[test]
    fn test_next_token_percent_ai_ms() {
        let mut lexer = Lexer::new("%ms");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Percent(_)));
        assert_eq!(token.to_string(), "%ms");
    }

    #[test]
    fn test_next_token_percent_ai_question() {
        let mut lexer = Lexer::new("%m?");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Percent(_)));
        assert_eq!(token.to_string(), "%m?");
    }

    #[test]
    fn test_next_token_utf8_text() {
        let mut lexer = Lexer::new("さくら🌸うにゅう");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Text(_)));
        assert_eq!(token.to_string(), "さくら🌸うにゅう");
    }

    #[test]
    fn test_next_token_whitespace_spaces() {
        let mut lexer = Lexer::new("   ");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Text(_)));
        assert_eq!(token.to_string(), "   ");
    }

    #[test]
    fn test_next_token_whitespace_mixed() {
        let mut lexer = Lexer::new("\t\r\n");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Text(_)));
        assert_eq!(token.to_string(), "\t\r\n");
    }

    #[test]
    fn test_next_token_special_characters() {
        let mut lexer = Lexer::new("!@#$^&*()_+-={}|:\";<>?./");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Text(_)));
        assert_eq!(token.to_string(), "!@#$^&*()_+-={}|:\";<>?./");
    }

    #[test]
    fn test_next_token_consecutive_simple() {
        let mut lexer = Lexer::new("\\h[0]");
        let token1 = lexer.next_token().unwrap();
        assert!(matches!(token1, Token::Escape(_)));
        assert_eq!(token1.to_string(), "\\h");
        let token2 = lexer.next_token().unwrap();
        assert!(matches!(token2, Token::Bracket(_)));
        assert_eq!(token2.to_string(), "[");
        let token3 = lexer.next_token().unwrap();
        assert!(matches!(token3, Token::Text(_)));
        assert_eq!(token3.to_string(), "0");
        let token4 = lexer.next_token().unwrap();
        assert!(matches!(token4, Token::Bracket(_)));
        assert_eq!(token4.to_string(), "]");
    }

    #[test]
    fn test_next_token_bracket_content() {
        let mut lexer = Lexer::new("[10]");
        let token1 = lexer.next_token().unwrap();
        assert!(matches!(token1, Token::Bracket(_)));
        let token2 = lexer.next_token().unwrap();
        assert!(matches!(token2, Token::Text(_)));
        assert_eq!(token2.to_string(), "10");
        let token3 = lexer.next_token().unwrap();
        assert!(matches!(token3, Token::Bracket(_)));
    }

    #[test]
    fn test_next_token_percent_unknown() {
        let mut lexer = Lexer::new("%unknown");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Percent(_)));
        assert_eq!(token.to_string(), "%unknown");
    }

    #[test]
    fn test_next_token_illegal_backslash() {
        let mut lexer = Lexer::new("\\");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Illegal(_)));
        assert_eq!(token.to_string(), "\\");
    }

    #[test]
    fn test_next_token_illegal_percent() {
        let mut lexer = Lexer::new("%");
        let token = lexer.next_token().unwrap();
        assert!(matches!(token, Token::Illegal(_)));
        assert_eq!(token.to_string(), "%");
    }

    #[test]
    fn test_next_token_illegal_backslash_at_end() {
        let mut lexer = Lexer::new("hello\\");
        let token1 = lexer.next_token().unwrap();
        assert!(matches!(token1, Token::Text(_)));
        assert_eq!(token1.to_string(), "hello");
        let token2 = lexer.next_token().unwrap();
        assert!(matches!(token2, Token::Illegal(_)));
        assert_eq!(token2.to_string(), "\\");
    }

    #[test]
    fn test_next_token_illegal_percent_at_end() {
        let mut lexer = Lexer::new("world%");
        let token1 = lexer.next_token().unwrap();
        assert!(matches!(token1, Token::Text(_)));
        assert_eq!(token1.to_string(), "world");
        let token2 = lexer.next_token().unwrap();
        assert!(matches!(token2, Token::Illegal(_)));
        assert_eq!(token2.to_string(), "%");
    }

    #[test]
    fn test_next_token_illegal_percent_invalid_char() {
        let mut lexer = Lexer::new("%@");
        let token1 = lexer.next_token().unwrap();
        assert!(matches!(token1, Token::Illegal(_)));
        assert_eq!(token1.to_string(), "%");
        let token2 = lexer.next_token().unwrap();
        assert!(matches!(token2, Token::Text(_)));
        assert_eq!(token2.to_string(), "@");
    }
}
