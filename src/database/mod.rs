use mongodb::{Client, Database};

use crate::setting;

pub async fn connect(cfg: &setting::Database) -> Database {
    Client::with_uri_str(cfg.address())
        .await
        .expect("mongodb connection failed")
        .database(cfg.name())
}
