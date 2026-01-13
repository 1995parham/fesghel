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
