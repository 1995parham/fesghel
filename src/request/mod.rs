use serde::Deserialize;
// `as` keyword creates an alias to avoid name collision with our `Url` struct.
use url::Url as ParsedUrl;

// Only `Deserialize` needed - this struct receives data, never sends it.
#[derive(Debug, Deserialize)]
pub struct Url {
    url: String,
    // `Option<T>` represents an optional value: Some(value) or None.
    // Serde treats missing JSON fields as None for Option types.
    name: Option<String>,
}

// `enum` in Rust is an algebraic data type (sum type).
// Each variant can hold different data - more powerful than C enums.
#[derive(Debug)]
pub enum ValidationError {
    // Variant holding associated data (the parse error).
    // This pattern enables rich error types with context.
    InvalidUrl(url::ParseError),
}

// Implementing Display for custom error messages.
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // `match` on enum must handle all variants (exhaustive).
        // Compiler error if you forget a variant - prevents bugs.
        match self {
            // Pattern destructuring: extracts `e` from the variant.
            ValidationError::InvalidUrl(e) => write!(f, "invalid URL: {}", e),
        }
    }
}

impl Url {
    pub fn validate(&self) -> Result<(), ValidationError> {
        // `?` operator: if Result is Err, return early with that error.
        // `map_err` converts the error type before `?` propagates it.
        // Here: ParseError -> ValidationError::InvalidUrl(ParseError).
        ParsedUrl::parse(&self.url).map_err(ValidationError::InvalidUrl)?;
        Ok(())
    }

    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    pub fn name(&self) -> &str {
        // `as_deref()` converts Option<String> to Option<&str>.
        // `unwrap_or` returns the inner value or a default if None.
        // Combined: returns &str of name, or "-" if name is None.
        self.name.as_deref().unwrap_or("-")
    }
}
