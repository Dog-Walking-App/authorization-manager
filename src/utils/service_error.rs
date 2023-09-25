use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ServiceError {
    message: String,
}

impl ServiceError {
    pub fn new(message: &str) -> ServiceError {
        ServiceError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ServiceError {
    fn description(&self) -> &str {
        &self.message
    }
}
