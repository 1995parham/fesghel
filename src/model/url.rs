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
