use std::error;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub error: Box<dyn error::Error>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "store error: {}", self.error)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&*self.error)
    }
}
