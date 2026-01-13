use mongodb::{Client, Database};

use crate::setting;

// Free function (not associated with a type) for database connection.
// `&setting::Database` borrows the config - we only need to read it.
pub async fn connect(cfg: &setting::Database) -> Database {
    // Method chaining: each method returns a value for the next call.
    // `with_uri_str` returns a Future that resolves to Result<Client, Error>.
    Client::with_uri_str(cfg.address())
        .await
        // `expect()` unwraps Result, panicking with message if Err.
        // Acceptable here since we can't proceed without database.
        .expect("mongodb connection failed")
        // `database()` selects a database by name from the client.
        .database(cfg.name())
}
