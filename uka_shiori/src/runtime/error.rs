use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

/// `ShioriError` is a custom error type used across the SHIORI service.
///
/// This structure provides a standardized way to wrap and handle various
/// kinds of errors that may occur during the processing of a SHIORI request.
/// It encapsulates a source error and optionally provides additional context information.
pub struct ShioriError {
    /// The source error which is wrapped by `ShioriError`.
    source: Box<dyn std::error::Error + Send + Sync>,

    /// Optional context information associated with the error.
    context: Option<String>,
}

impl ShioriError {
    /// Constructs a new `ShioriError` by wrapping an error object.
    ///
    /// The provided error is stored as the source error of the `ShioriError`.
    /// The source error must implement the `Error` trait and be thread safe.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use uka_shiori::runtime::ShioriError;
    /// #
    /// let e = ShioriError::new("something went wrong");
    /// assert_eq!(format!("{e}"), "something went wrong");
    /// ```
    pub fn new(e: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> Self {
        Self {
            source: e.into(),
            context: None,
        }
    }

    /// Creates a new `ShioriError` from a specific source error.
    ///
    /// This is similar to the `new` method, but allows for more specific type
    /// information on the source error.
    pub fn source<E: std::error::Error + Send + Sync + 'static>(error: E) -> Self {
        Self {
            source: Box::new(error),
            context: None,
        }
    }

    /// Attaches a context message to the `ShioriError`.
    ///
    /// If there is an existing context, the new context will be appended to it.
    /// The context message can help provide additional information about where or
    /// why the error occurred.
    pub(crate) fn context<C: Display>(mut self, context: C) -> Self {
        match self.context {
            Some(ref mut c) => {
                c.push_str(": ");
                c.push_str(&context.to_string());
            }
            None => {
                self.context = Some(context.to_string());
            }
        };
        self
    }
}

impl Deref for ShioriError {
    type Target = dyn std::error::Error + Send + Sync;

    fn deref(&self) -> &Self::Target {
        self.source.as_ref()
    }
}

impl Display for ShioriError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.context {
            Some(c) => {
                let e = self.source.deref();
                write!(f, "{c}: {e}")
            }
            None => {
                let e = self.source.deref();
                write!(f, "{e}")
            }
        }
    }
}

impl Debug for ShioriError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.context {
            Some(c) => {
                let e = self.source.deref();
                write!(f, "{c}: {e:?}")
            }
            None => {
                let e = self.source.deref();
                write!(f, "{e:?}")
            }
        }
    }
}

impl<E> From<E> for ShioriError
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(value: E) -> Self {
        ShioriError::source(value)
    }
}

impl From<ShioriError> for Box<dyn std::error::Error + Send + Sync> {
    fn from(value: ShioriError) -> Self {
        value.source
    }
}

impl From<ShioriError> for Box<dyn std::error::Error + Send> {
    fn from(value: ShioriError) -> Self {
        value.source
    }
}

impl From<ShioriError> for Box<dyn std::error::Error + Sync> {
    fn from(value: ShioriError) -> Self {
        value.source
    }
}

impl From<ShioriError> for Box<dyn std::error::Error> {
    fn from(value: ShioriError) -> Self {
        value.source
    }
}

/// `ShioriErrorContext` provides extension methods to attach additional context information
/// to the `ShioriError`.
///
/// This trait is used to enrich `ShioriError` instances with extra context information,
/// which can be useful for debugging and error reporting.
pub trait ShioriErrorContext {
    /// Attaches a context message to the `ShioriError`.
    ///
    /// This message is displayed alongside the original error message.
    fn context(self, context: impl Into<String>) -> Self;

    /// Attaches a context message to the `ShioriError`, where the context is produced by a function.
    ///
    /// This method can be useful when the context information requires some computation
    /// to produce, as the function will only be called if there is an error.
    fn with_context<F, I>(self, f: F) -> Self
    where
        F: FnOnce() -> I,
        I: Into<String>;
}

impl<T> ShioriErrorContext for Result<T, ShioriError> {
    fn context(self, context: impl Into<String>) -> Self {
        self.map_err(|e| e.context(context.into()))
    }

    fn with_context<F, I>(self, f: F) -> Self
    where
        F: FnOnce() -> I,
        I: Into<String>,
    {
        self.map_err(|e| e.context(f().into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(thiserror::Error, Debug)]
    #[error("Test error")]
    struct TestError;

    #[test]
    fn test_service_error_new() {
        let e = ShioriError::new("Test error");
        assert_eq!(e.to_string(), "Test error");
    }

    #[test]
    fn test_service_error_source() {
        let e = ShioriError::source(TestError);
        assert_eq!(e.to_string(), "Test error");
    }

    #[test]
    fn test_service_error_context() {
        let r: Result<(), ShioriError> =
            Err(ShioriError::source(TestError)).context("Extra context");
        assert!(r.is_err());

        let e = r.unwrap_err();
        assert_eq!(format!("{e}"), "Extra context: Test error");
    }

    #[test]
    fn test_service_error_with_context() {
        let r: Result<(), ShioriError> =
            Err(ShioriError::new(TestError)).with_context(|| "Some context from closure");

        let e = r.unwrap_err();
        assert_eq!(format!("{e}"), "Some context from closure: Test error");
    }

    #[test]
    fn test_service_error_set_multiple_contexts() {
        let e = ShioriError::new(TestError)
            .context("First context")
            .context("Second context");
        assert_eq!(e.to_string(), "First context: Second context: Test error");
    }

    #[test]
    fn test_display_and_debug() {
        let e = ShioriError::new(TestError).context("Extra context");
        assert_eq!(format!("{e}"), "Extra context: Test error");
        assert_eq!(format!("{e:?}"), "Extra context: TestError");
    }
}
