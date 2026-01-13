// `std::error` and `std::fmt` are standard library modules.
// Rust's standard library provides core functionality.
use std::error;
use std::fmt;

// `#[derive(Debug)]` auto-implements the Debug trait for printing with `{:?}`.
// Required for types used in error messages and debugging.
#[derive(Debug)]
pub struct Error {
    // `Box<dyn Trait>` is a trait object - a pointer to any type implementing Trait.
    // `dyn` indicates dynamic dispatch (runtime polymorphism via vtable).
    // `Box` provides heap allocation for the dynamically-sized trait object.
    pub error: Box<dyn error::Error>,
}

// Implementing `Display` trait allows formatting with `{}` in format strings.
// This is how errors get their human-readable message.
impl fmt::Display for Error {
    // `&self` borrows self, `&mut fmt::Formatter` is the output buffer.
    // `'_` is an elided lifetime - compiler infers it automatically.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // `write!` macro writes formatted text to a formatter or buffer.
        write!(f, "store error: {}", self.error)
    }
}

// Implementing `std::error::Error` trait makes this a proper Rust error type.
// Enables use with `?` operator and error handling ecosystem.
impl error::Error for Error {
    // `source()` returns the underlying cause of the error (error chaining).
    // `'static` lifetime means the error doesn't borrow temporary data.
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // `&*self.error` dereferences Box then re-borrows: Box<T> -> T -> &T
        Some(&*self.error)
    }
}
