use mongodb::bson::doc;
use mongodb::error::{ErrorKind, WriteFailure};
use mongodb::{Collection, Database, IndexModel};
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
    // `async fn` returns a Future. Use when method performs I/O operations.
    // This constructor is async because it creates database indexes.
    pub async fn new(db: Database) -> Self {
        let collection = db.collection(COLLECTION);

        // Create a unique index on `key` field.
        // MongoDB will reject inserts with duplicate keys (error code 11000).
        let index = IndexModel::builder()
            .keys(doc! { "key": 1 })
            .options(
                mongodb::options::IndexOptions::builder()
                    .unique(true)
                    .build(),
            )
            .build();

        // `let _ = ...` explicitly ignores the Result.
        // Index creation may fail if index already exists - that's OK.
        let _ = collection.create_index(index).await;

        Url { collection }
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
            .find_one(doc! { "key": name })
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
            .map_err(|err| {
                // Check if this is a duplicate key error (MongoDB error code 11000).
                // Pattern matching on nested enum variants to extract error details.
                if let ErrorKind::Write(WriteFailure::WriteError(write_error)) = err.kind.as_ref() {
                    if write_error.code == 11000 {
                        return Error::DuplicateKey(url.key().to_string());
                    }
                }
                // Wrap other errors in the Database variant.
                Error::Database(Box::new(err))
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
