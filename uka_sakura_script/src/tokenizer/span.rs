/// Span represents a range of bytes in the original source text.
///
/// Spans are used throughout the tokenizer to track the precise location of tokens
/// and errors in the original Sakura Script source. This enables accurate error
/// reporting and source mapping for debugging tools.
///
/// # Examples
///
/// ```rust
/// # use uka_sakura_script::Span;
/// let span = Span::new(10, 15);
/// assert_eq!(span.len(), 5);
/// assert!(!span.is_empty());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    start: usize,

    end: usize,
}

impl Span {
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub const fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    pub const fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    /// Merges this span with another span, returning a span that covers both.
    ///
    /// The resulting span will start at the minimum start position and end at
    /// the maximum end position of the two input spans.
    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}
