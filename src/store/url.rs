use mongodb::{Collection, Database, bson};
use rand::{Rng, distr::Alphanumeric, rng};

// `super::` refers to the parent module. Here it accesses `store::error`.
use super::error::Error;
use crate::model;

// `const` defines a compile-time constant. Must have explicit type annotation.
// `&str` is a string slice - a reference to string data with known length.
const COLLECTION: &str = "urls";
// `usize` is pointer-sized unsigned integer, used for indexing and lengths.
const LENGTH: usize = 6;

// `#[derive]` is an attribute that auto-implements traits.
// `Clone` trait allows creating a deep copy via `.clone()` method.
#[derive(Clone)]
pub struct Url {
    // `Collection<T>` is a generic type - T specifies the document type.
    // Generics enable type-safe code reuse without runtime overhead.
    collection: Collection<model::Url>,
}

impl Url {
    pub fn new(db: Database) -> Self {
        Url {
            collection: db.collection(COLLECTION),
        }
    }

    pub fn random_key() -> String {
        // Method chaining with iterators - a functional programming pattern.
        // Each method transforms the iterator, evaluated lazily until `collect()`.
        rng()
            // `sample_iter` creates an infinite iterator of random samples.
            .sample_iter(&Alphanumeric)
            // `take(n)` limits iterator to first n elements.
            .take(LENGTH)
            // `map` transforms each element. `char::from` converts u8 to char.
            .map(char::from)
            // `collect()` consumes iterator and builds a collection.
            // Type inference determines we want String from context.
            .collect()
    }

    // `&self` borrows self immutably - method can read but not modify.
    // `&str` is a borrowed string slice - avoids copying the string data.
    pub async fn fetch(&self, name: &str) -> Option<model::Url> {
        self.collection
            // `doc!` macro creates BSON documents with JSON-like syntax.
            .find_one(bson::doc! { "key": name })
            .await
            // `.ok()` converts Result<T, E> to Option<T>, discarding errors.
            .ok()
            // `.flatten()` converts Option<Option<T>> to Option<T>.
            .flatten()
    }

    // `&model::Url` borrows the URL - we don't take ownership.
    // `Result<(), Error>` returns either success (unit type `()`) or an Error.
    pub async fn store(&self, url: &model::Url) -> Result<(), Error> {
        self.collection
            .insert_one(url)
            .await
            // `map_err` transforms the error type, leaving Ok unchanged.
            // Closure `|err|` captures the error and wraps it in our Error type.
            .map_err(|err| Error {
                // `Box::new()` allocates on heap. Required for trait objects.
                error: Box::new(err),
            })
            // `map` transforms the Ok value. `|_|` ignores the input.
            // `()` is the unit type - similar to void but is an actual value.
            .map(|_| ())
    }
}

// `#[cfg(test)]` is conditional compilation - this module only exists in test builds.
// Keeps test code out of production binary.
#[cfg(test)]
mod tests {
    // `#[test]` marks a function as a test case, run via `cargo test`.
    #[test]
    fn random_key() {
        // `super::` accesses parent module (the outer `Url` impl).
        let s1 = super::Url::random_key();
        // `assert_eq!` macro panics if values aren't equal, failing the test.
        assert_eq!(s1.len(), 6);

        let s2 = super::Url::random_key();

        // `assert_ne!` panics if values ARE equal.
        assert_ne!(s1, s2);
    }
}
