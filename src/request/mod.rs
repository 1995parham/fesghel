use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct URL {
    url: String,
    name: Option<String>,
    expire: Option<DateTime<Utc>>,
}

impl URL {
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
