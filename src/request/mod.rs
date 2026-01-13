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

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create Url for testing.
    // In tests, we often need to construct structs that don't have public constructors.
    fn make_url(url: &str, name: Option<&str>) -> Url {
        Url {
            url: url.to_string(),
            name: name.map(String::from),
        }
    }

    #[test]
    fn validate_valid_https_url() {
        let url = make_url("https://example.com", None);
        assert!(url.validate().is_ok());
    }

    #[test]
    fn validate_valid_http_url() {
        let url = make_url("http://example.com/path?query=1", None);
        assert!(url.validate().is_ok());
    }

    #[test]
    fn validate_valid_url_with_port() {
        let url = make_url("https://localhost:8080/api", None);
        assert!(url.validate().is_ok());
    }

    #[test]
    fn validate_invalid_url_no_scheme() {
        let url = make_url("example.com", None);
        assert!(url.validate().is_err());
    }

    #[test]
    fn validate_invalid_url_empty() {
        let url = make_url("", None);
        assert!(url.validate().is_err());
    }

    #[test]
    fn validate_invalid_url_random_string() {
        let url = make_url("not a url at all", None);
        assert!(url.validate().is_err());
    }

    #[test]
    fn name_returns_value_when_present() {
        let url = make_url("https://example.com", Some("mykey"));
        assert_eq!(url.name(), "mykey");
    }

    #[test]
    fn name_returns_dash_when_none() {
        let url = make_url("https://example.com", None);
        assert_eq!(url.name(), "-");
    }

    #[test]
    fn url_returns_the_url_string() {
        let url = make_url("https://example.com/path", Some("key"));
        assert_eq!(url.url(), "https://example.com/path");
    }

    #[test]
    fn validation_error_display() {
        let url = make_url("invalid", None);
        let err = url.validate().unwrap_err();
        // `to_string()` uses the Display trait implementation.
        assert!(err.to_string().contains("invalid URL"));
    }
}
