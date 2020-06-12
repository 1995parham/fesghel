use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct URL {
    actual: String,
    short: String,
}

impl URL {
    pub fn new(actual: &str, short: &str) -> Self {
        return URL {
            actual: String::from(actual),
            short: String::from(short),
        };
    }
}
