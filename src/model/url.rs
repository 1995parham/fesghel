// Serde is Rust's serialization framework.
// `Serialize` converts Rust types to formats like JSON/BSON.
// `Deserialize` converts JSON/BSON back to Rust types.
use serde::{Deserialize, Serialize};

// Multiple derives can be combined in one attribute.
// `Debug` - enables `{:?}` formatting for debugging
// `Serialize` - enables conversion TO JSON/BSON (for responses)
// `Deserialize` - enables conversion FROM JSON/BSON (from database)
#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
    // `String` is an owned, heap-allocated, growable string.
    // Fields are private by default - only accessible within this module.
    url: String,
    key: String,
}

impl Url {
    // Getter methods return `&str` (borrowed slice) instead of `String`.
    // This avoids copying and is idiomatic for read-only access.
    pub fn key(&self) -> &str {
        // `as_str()` converts String to &str (borrows the string data).
        self.key.as_str()
    }

    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    // Constructor pattern: `new()` is conventional name for creating instances.
    // Takes `&str` (borrowed) and converts to owned `String` internally.
    pub fn new(url: &str, key: &str) -> Self {
        Url {
            // `String::from()` creates an owned String from a string slice.
            // This allocates memory and copies the string data.
            url: String::from(url),
            key: String::from(key),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_url_with_correct_values() {
        let url = Url::new("https://example.com", "abc123");
        assert_eq!(url.url(), "https://example.com");
        assert_eq!(url.key(), "abc123");
    }

    #[test]
    fn new_handles_empty_strings() {
        let url = Url::new("", "");
        assert_eq!(url.url(), "");
        assert_eq!(url.key(), "");
    }

    #[test]
    fn new_handles_unicode() {
        let url = Url::new("https://例え.jp/パス", "キー");
        assert_eq!(url.url(), "https://例え.jp/パス");
        assert_eq!(url.key(), "キー");
    }

    #[test]
    fn serialize_to_json() {
        let url = Url::new("https://example.com", "test");
        // `serde_json::to_string` serializes to JSON string.
        let json = serde_json::to_string(&url).unwrap();
        assert!(json.contains("\"url\":\"https://example.com\""));
        assert!(json.contains("\"key\":\"test\""));
    }

    #[test]
    fn deserialize_from_json() {
        let json = r#"{"url":"https://example.com","key":"mykey"}"#;
        // `serde_json::from_str` deserializes from JSON string.
        let url: Url = serde_json::from_str(json).unwrap();
        assert_eq!(url.url(), "https://example.com");
        assert_eq!(url.key(), "mykey");
    }

    #[test]
    fn serialize_deserialize_roundtrip() {
        let original = Url::new("https://test.com/path?q=1", "key123");
        let json = serde_json::to_string(&original).unwrap();
        let restored: Url = serde_json::from_str(&json).unwrap();
        assert_eq!(original.url(), restored.url());
        assert_eq!(original.key(), restored.key());
    }

    #[test]
    fn debug_format() {
        let url = Url::new("https://example.com", "key");
        // `format!("{:?}", ...)` uses Debug trait.
        let debug = format!("{:?}", url);
        assert!(debug.contains("Url"));
        assert!(debug.contains("https://example.com"));
        assert!(debug.contains("key"));
    }
}
