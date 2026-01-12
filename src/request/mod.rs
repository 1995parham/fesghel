use serde::Deserialize;
use url::Url as ParsedUrl;

#[derive(Debug, Deserialize)]
pub struct Url {
    url: String,
    name: Option<String>,
}

#[derive(Debug)]
pub enum ValidationError {
    InvalidUrl(url::ParseError),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidUrl(e) => write!(f, "invalid URL: {}", e),
        }
    }
}

impl Url {
    pub fn validate(&self) -> Result<(), ValidationError> {
        ParsedUrl::parse(&self.url).map_err(ValidationError::InvalidUrl)?;
        Ok(())
    }

    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_deref().unwrap_or("-")
    }
}
