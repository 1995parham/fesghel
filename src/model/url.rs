use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct URL {
    url: String,
    name: String,
}

impl URL {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    pub fn new(url: &str, name: &str) -> Self {
        URL {
            url: String::from(url),
            name: String::from(name),
        }
    }
}
