use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct URL {
    url: String,
    key: String,
}

impl URL {
    pub fn key(&self) -> &str {
        self.key.as_str()
    }

    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    pub fn new(url: &str, key: &str) -> Self {
        URL {
            url: String::from(url),
            key: String::from(key),
        }
    }
}
