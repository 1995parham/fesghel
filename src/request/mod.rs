// use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Url {
    url: String,
    name: Option<String>,
    // expire: Option<DateTime<Utc>>,
}

impl Url {
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    pub fn name(&self) -> &str {
        if let Some(s) = &self.name {
            s.as_str()
        } else {
            "-"
        }
    }
}
