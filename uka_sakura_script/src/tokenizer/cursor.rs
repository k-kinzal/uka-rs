/// A cursor for efficiently navigating through UTF-8 encoded string data.
///
/// `Cursor` provides a stateful iterator-like interface for reading and examining
/// characters from a string source while maintaining proper UTF-8 character boundaries.
/// It tracks the current position in terms of character count (not byte position)
/// and provides methods for reading, peeking, and conditional reading operations.
///
/// # Examples
///
/// ```rust
/// # use uka_sakura_script::tokenizer::Cursor;
/// let mut cursor = Cursor::new("Hello, 世界!");
///
/// // Read individual characters
/// assert_eq!(cursor.next(), Some('H'));
/// assert_eq!(cursor.next(), Some('e'));
///
/// // Check current position (character-based)
/// assert_eq!(cursor.position(), 2);
///
/// // Peek at the next character without advancing
/// assert_eq!(cursor.peek(), Some('l'));
/// assert_eq!(cursor.position(), 2); // Position unchanged
///
/// // Read multiple characters at once
/// let chars = cursor.read(3);
/// assert_eq!(chars, "llo");
/// assert_eq!(cursor.position(), 5);
/// ```
pub struct Cursor<'a> {
    inner: &'a str,
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(inner: &'a str) -> Self {
        Self { inner, pos: 0 }
    }

    pub fn into_inner(self) -> &'a str {
        self.inner
    }

    /// Returns the current position of the cursor in terms of character count.
    pub fn position(&self) -> usize {
        self.inner[..self.pos].chars().count()
    }

    /// Reads up to `n` characters from the current position and advances the cursor.
    pub fn read(&mut self, n: usize) -> &'a str {
        let start = self.pos;
        let size = self.inner[self.pos..]
            .chars()
            .take(n)
            .map(|c| c.len_utf8())
            .sum::<usize>();
        let end = (self.pos + size).min(self.inner.len());

        self.pos = end;
        &self.inner[start..end]
    }

    /// Reads characters while the given predicate returns `true` and advances the cursor.
    pub fn read_while<P>(&mut self, predicate: P) -> &'a str
    where
        P: Fn(char) -> bool,
    {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if !predicate(c) {
                break;
            }
            self.next();
        }
        &self.inner[start..self.pos]
    }

    /// Returns the next character without advancing the cursor.(), 1);
    /// ```
    pub fn peek(&self) -> Option<char> {
        self.inner[self.pos..].chars().next()
    }

    /// Returns a string slice containing up to `n` characters starting from the current position.
    pub fn peek_nth(&self, n: usize) -> &'a str {
        let size = self.inner[self.pos..]
            .chars()
            .take(n)
            .map(|c| c.len_utf8())
            .sum::<usize>();
        let end = (self.pos + size).min(self.inner.len());

        &self.inner[self.pos..end]
    }
}

/// Iterator implementation for `Cursor`.
///
/// This allows the cursor to be used in for loops and with iterator methods.
/// Each call to `next()` advances the cursor by one character and returns that character.
/// The iteration respects UTF-8 character boundaries.
///
/// # Examples
///
/// ```rust
/// # use uka_sakura_script::tokenizer::Cursor;
/// let mut cursor = Cursor::new("Hello");
/// let chars: Vec<char> = cursor.collect();
/// assert_eq!(chars, vec!['H', 'e', 'l', 'l', 'o']);
/// ```
impl Iterator for Cursor<'_> {
    type Item = char;

    /// Advances the cursor by one character and returns it.
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.inner[self.pos..].chars().next()?;
        self.pos += c.len_utf8();
        Some(c)
    }
}

#[cfg(test)]
mod tests {
    use super::Cursor;

    #[test]
    fn test_position() {
        let mut cursor = Cursor::new("hello");
        assert_eq!(cursor.position(), 0);
        cursor.next();
        assert_eq!(cursor.position(), 1);
        cursor.next();
        assert_eq!(cursor.position(), 2);
        cursor.read(2);
        assert_eq!(cursor.position(), 4);
        cursor.next();
        assert_eq!(cursor.position(), 5);
        assert_eq!(cursor.next(), None);
        assert_eq!(cursor.position(), 5);
    }

    #[test]
    fn test_position_utf8_tracking() {
        let mut cursor = Cursor::new("a🌟b");
        assert_eq!(cursor.position(), 0);
        cursor.next();
        assert_eq!(cursor.position(), 1);
        cursor.next();
        assert_eq!(cursor.position(), 2);
        cursor.next();
        assert_eq!(cursor.position(), 3);
    }

    #[test]
    fn test_position_consistency_across_operations() {
        let mut cursor = Cursor::new("test");
        let initial_pos = cursor.position();

        cursor.peek();
        assert_eq!(cursor.position(), initial_pos);

        cursor.peek_nth(10);
        assert_eq!(cursor.position(), initial_pos);

        cursor.next();
        assert!(cursor.position() > initial_pos);
    }

    #[test]
    fn test_next() {
        let mut cursor = Cursor::new("hello");
        assert_eq!(cursor.next(), Some('h'));
        assert_eq!(cursor.next(), Some('e'));
        assert_eq!(cursor.next(), Some('l'));
        assert_eq!(cursor.next(), Some('l'));
        assert_eq!(cursor.next(), Some('o'));
        assert_eq!(cursor.next(), None);
    }

    #[test]
    fn test_next_utf8_multibyte_characters() {
        let mut cursor = Cursor::new("Hello 🌟 世界");
        assert_eq!(cursor.next(), Some('H'));
        assert_eq!(cursor.next(), Some('e'));
        assert_eq!(cursor.next(), Some('l'));
        assert_eq!(cursor.next(), Some('l'));
        assert_eq!(cursor.next(), Some('o'));
        assert_eq!(cursor.next(), Some(' '));
        assert_eq!(cursor.next(), Some('🌟'));
        assert_eq!(cursor.next(), Some(' '));
        assert_eq!(cursor.next(), Some('世'));
        assert_eq!(cursor.next(), Some('界'));
        assert_eq!(cursor.next(), None);
    }

    #[test]
    fn test_next_single_character() {
        let mut cursor = Cursor::new("x");
        assert_eq!(cursor.next(), Some('x'));
        assert_eq!(cursor.next(), None);
    }

    #[test]
    fn test_next_control_characters() {
        let mut cursor = Cursor::new("a\tb\0c");
        assert_eq!(cursor.next(), Some('a'));
        assert_eq!(cursor.next(), Some('\t'));
        assert_eq!(cursor.next(), Some('b'));
        assert_eq!(cursor.next(), Some('\0'));
        assert_eq!(cursor.next(), Some('c'));
    }

    #[test]
    fn test_next_operations_at_end() {
        let mut cursor = Cursor::new("ab");
        cursor.next();
        cursor.next();

        assert_eq!(cursor.next(), None);
    }

    #[test]
    fn test_next_state_after_exhaustion() {
        let mut cursor = Cursor::new("abc");

        while cursor.next().is_some() {}

        assert_eq!(cursor.next(), None);
    }

    #[test]
    fn test_read() {
        let mut cursor = Cursor::new("hello");
        assert_eq!(cursor.read(2), "he");
        assert_eq!(cursor.read(2), "ll");
        assert_eq!(cursor.read(2), "o");
        assert_eq!(cursor.read(2), "");
    }

    #[test]
    fn test_read_utf8_operations() {
        let mut cursor = Cursor::new("café");
        assert_eq!(cursor.read(3), "caf");
        assert_eq!(cursor.read(2), "é");
    }

    #[test]
    fn test_read_more_than_available() {
        let mut cursor = Cursor::new("hi");
        assert_eq!(cursor.read(10), "hi");
        assert_eq!(cursor.read(5), "");
    }

    #[test]
    fn test_read_large_string_operations() {
        let large_string = "a".repeat(10000);
        let mut cursor = Cursor::new(&large_string);

        let first_half = cursor.read(5000);
        assert_eq!(first_half.len(), 5000);

        let second_half = cursor.read(10000);
        assert_eq!(second_half.len(), 5000);
    }

    #[test]
    fn test_read_while() {
        let mut cursor = Cursor::new("hello123");
        assert_eq!(cursor.read_while(|c| c.is_alphabetic()), "hello");
        assert_eq!(cursor.read_while(|c| c.is_numeric()), "123");
        assert_eq!(cursor.read_while(|c| c.is_alphabetic()), "");
    }

    #[test]
    fn test_read_while_multiple_calls() {
        let mut cursor = Cursor::new("abc123xyz789");
        assert_eq!(cursor.read_while(|c| c.is_alphabetic()), "abc");
        assert_eq!(cursor.read_while(|c| c.is_numeric()), "123");
        assert_eq!(cursor.read_while(|c| c.is_alphabetic()), "xyz");
        assert_eq!(cursor.read_while(|c| c.is_numeric()), "789");
        assert_eq!(cursor.read_while(|c| c.is_alphabetic()), "");
    }

    #[test]
    fn test_read_while_complex_predicates() {
        let mut cursor = Cursor::new("Hello-World_123!");
        assert_eq!(
            cursor.read_while(|c| c.is_alphanumeric() || c == '-' || c == '_'),
            "Hello-World_123"
        );
    }

    #[test]
    fn test_read_while_whitespace_only() {
        let mut cursor = Cursor::new("   \t\n");
        assert_eq!(cursor.read_while(|c| c.is_whitespace()), "   \t\n");
    }

    #[test]
    fn test_read_while_line_endings() {
        let mut cursor = Cursor::new("line1\nline2\r\nline3");
        assert_eq!(cursor.read_while(|c| c != '\n' && c != '\r'), "line1");
        assert_eq!(cursor.next(), Some('\n'));
        assert_eq!(cursor.read_while(|c| c != '\n' && c != '\r'), "line2");
        assert_eq!(cursor.next(), Some('\r'));
        assert_eq!(cursor.next(), Some('\n'));
        assert_eq!(cursor.read_while(|c| c != '\n' && c != '\r'), "line3");
    }

    #[test]
    fn test_read_while_mixed_ascii_unicode() {
        let mut cursor = Cursor::new("Hello🌟World世界!");
        assert_eq!(cursor.read_while(|c| c.is_ascii_alphabetic()), "Hello");
        cursor.next();
        assert_eq!(cursor.read_while(|c| c.is_ascii_alphabetic()), "World");
        assert_eq!(cursor.read_while(|c| !c.is_ascii_punctuation()), "世界");
    }

    #[test]
    fn test_read_while_empty_result() {
        let mut cursor = Cursor::new("123abc");
        assert_eq!(cursor.read_while(|c| c.is_alphabetic()), "");
    }

    #[test]
    fn test_peek() {
        let mut cursor = Cursor::new("hello");
        assert_eq!(cursor.peek(), Some('h'));
        cursor.next();
        assert_eq!(cursor.peek(), Some('e'));
        cursor.read(2);
        assert_eq!(cursor.peek(), Some('l'));
        cursor.next();
        assert_eq!(cursor.peek(), Some('o'));
        cursor.next();
        assert_eq!(cursor.peek(), None);
    }

    #[test]
    fn test_peek_utf8_operations() {
        let mut cursor = Cursor::new("🌟abc");
        assert_eq!(cursor.peek(), Some('🌟'));
        cursor.next();
        assert_eq!(cursor.peek(), Some('a'));
    }

    #[test]
    fn test_peek_nth() {
        let cursor = Cursor::new("hello");
        assert_eq!(cursor.peek_nth(1), "h");
        assert_eq!(cursor.peek_nth(2), "he");
        assert_eq!(cursor.peek_nth(5), "hello");
        assert_eq!(cursor.peek_nth(10), "hello");
    }

    #[test]
    fn test_peek_nth_beyond_length() {
        let cursor = Cursor::new("ab");
        assert_eq!(cursor.peek_nth(10), "ab");
        assert_eq!(cursor.peek_nth(0), "");
        assert_eq!(cursor.peek_nth(1), "a");
        assert_eq!(cursor.peek_nth(2), "ab");
    }

    #[test]
    fn test_peek_nth_utf8_emojis() {
        let cursor = Cursor::new("🌟🎉🚀");
        assert_eq!(cursor.peek_nth(1), "🌟");
        assert_eq!(cursor.peek_nth(2), "🌟🎉");
        assert_eq!(cursor.peek_nth(3), "🌟🎉🚀");
        assert_eq!(cursor.peek_nth(5), "🌟🎉🚀");
    }

    #[test]
    fn test_peek_nth_utf8_boundary_splitting() {
        let cursor = Cursor::new("a🌟b");
        assert_eq!(cursor.peek_nth(1), "a");
        assert_eq!(cursor.peek_nth(2), "a🌟");
        assert_eq!(cursor.peek_nth(3), "a🌟b");

        let mut cursor2 = Cursor::new("a🌟b");
        cursor2.next();
        assert_eq!(cursor2.peek_nth(1), "🌟");
        assert_eq!(cursor2.peek_nth(2), "🌟b");
    }
    #[test]
    fn test_peek_nth_utf8_character_boundaries() {
        let cursor = Cursor::new("a🌟b");
        assert_eq!(cursor.peek_nth(1), "a");
        assert_eq!(cursor.peek_nth(2), "a🌟");
        assert_eq!(cursor.peek_nth(3), "a🌟b");

        let mut cursor2 = Cursor::new("a🌟b");
        cursor2.next();
        assert_eq!(cursor2.peek_nth(1), "🌟");
        assert_eq!(cursor2.peek_nth(2), "🌟b");

        cursor2.next();
        assert_eq!(cursor2.peek_nth(1), "b");
    }

    #[test]
    fn test_peek_nth_mixed_utf8_content() {
        let cursor = Cursor::new("Hello🌟世界!");
        assert_eq!(cursor.peek_nth(5), "Hello");
        assert_eq!(cursor.peek_nth(6), "Hello🌟");
        assert_eq!(cursor.peek_nth(7), "Hello🌟世");
        assert_eq!(cursor.peek_nth(8), "Hello🌟世界");
        assert_eq!(cursor.peek_nth(9), "Hello🌟世界!");
        assert_eq!(cursor.peek_nth(15), "Hello🌟世界!");
    }

    #[test]
    fn test_peek_nth_after_cursor_movement() {
        let mut cursor = Cursor::new("a🌟b🎉c");
        assert_eq!(cursor.peek_nth(2), "a🌟");

        cursor.next();
        assert_eq!(cursor.peek_nth(1), "🌟");
        assert_eq!(cursor.peek_nth(2), "🌟b");

        cursor.next();
        assert_eq!(cursor.peek_nth(1), "b");
        assert_eq!(cursor.peek_nth(2), "b🎉");

        cursor.next();
        assert_eq!(cursor.peek_nth(1), "🎉");
        assert_eq!(cursor.peek_nth(2), "🎉c");
    }

    #[test]
    fn test_peek_nth_various_utf8_sizes() {
        let cursor = Cursor::new("aéं🌟");
        assert_eq!(cursor.peek_nth(1), "a");
        assert_eq!(cursor.peek_nth(2), "aé");
        assert_eq!(cursor.peek_nth(3), "aéं");
        assert_eq!(cursor.peek_nth(4), "aéं🌟");
    }
}
