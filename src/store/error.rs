use std::fmt;

// `enum` allows defining distinct error variants with different data.
// This is more expressive than a single struct - callers can match on variants.
#[derive(Debug)]
pub enum Error {
    // Variant for duplicate key errors (collision detection).
    // Contains the key that caused the collision.
    DuplicateKey(String),
    // Variant wrapping underlying database errors.
    // `Box<dyn ...>` is a trait object for any error type.
    Database(Box<dyn std::error::Error>),
}

impl Error {
    // Helper method to check if this error is a duplicate key error.
    // Useful for handlers to return appropriate HTTP status codes.
    pub fn is_duplicate_key(&self) -> bool {
        matches!(self, Error::DuplicateKey(_))
    }
}

// Implementing `Display` for human-readable error messages.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // `match` on enum variants to provide specific messages.
        match self {
            Error::DuplicateKey(key) => write!(f, "key already exists: {}", key),
            Error::Database(err) => write!(f, "database error: {}", err),
        }
    }
}

// Implementing `std::error::Error` for integration with error handling ecosystem.
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::DuplicateKey(_) => None,
            Error::Database(err) => Some(&**err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;
    use std::io;

    #[test]
    fn duplicate_key_is_duplicate_key() {
        let err = Error::DuplicateKey("test_key".to_string());
        assert!(err.is_duplicate_key());
    }

    #[test]
    fn database_error_is_not_duplicate_key() {
        let io_err = io::Error::new(io::ErrorKind::Other, "connection failed");
        let err = Error::Database(Box::new(io_err));
        assert!(!err.is_duplicate_key());
    }

    #[test]
    fn duplicate_key_display_message() {
        let err = Error::DuplicateKey("mykey".to_string());
        assert_eq!(err.to_string(), "key already exists: mykey");
    }

    #[test]
    fn database_error_display_message() {
        let io_err = io::Error::new(io::ErrorKind::Other, "connection refused");
        let err = Error::Database(Box::new(io_err));
        assert_eq!(err.to_string(), "database error: connection refused");
    }

    #[test]
    fn duplicate_key_has_no_source() {
        let err = Error::DuplicateKey("key".to_string());
        // `source()` returns the underlying cause of the error.
        assert!(err.source().is_none());
    }

    #[test]
    fn database_error_has_source() {
        let io_err = io::Error::new(io::ErrorKind::Other, "timeout");
        let err = Error::Database(Box::new(io_err));
        // Database errors should have a source (the wrapped error).
        assert!(err.source().is_some());
    }

    #[test]
    fn error_is_debug_printable() {
        let err = Error::DuplicateKey("debug_key".to_string());
        // Verify Debug trait is implemented and works.
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("DuplicateKey"));
        assert!(debug_str.contains("debug_key"));
    }

    #[test]
    fn error_implements_std_error() {
        // This test verifies at compile time that Error implements std::error::Error.
        fn assert_std_error<T: StdError>() {}
        assert_std_error::<Error>();
    }
}
