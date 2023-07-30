use std::error::Error;
use std::fmt;
use std::string::ToString;

use crate::util::location::SourceLocation;

///////////////////////////////////////////////////////////////////////////
/// This file contains implementation of the helper logic for error handling
/// used internally.
/// this comment is just to trigger the build
/// testing caching skills seeing if it will not install
///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct TError {
    message: String,
    location: SourceLocation,
}

#[allow(dead_code)]
impl TError {
    pub fn new(message: &str) -> Self {
        TError {
            message: message.to_string(),
            location: SourceLocation::new(0, 0),
        }
    }

    pub fn with_location(message: &str, location: SourceLocation) -> Self {
        TError {
            message: message.to_string(),
            location,
        }
    }

    // this function might not be necessary 
    pub fn get_error_message(&self) -> String {
        format!("{}{}", self.message, self.location.to_string())
    }

    /// Method to be used for checking required conditions.
    pub fn check(&self, check: bool, message: &str) -> Result<(), Box<dyn Error>> {
        if !check {
            Err(Box::new(TError::new(message)))
        } else {
            Ok(())
        }
    }
}

impl fmt::Display for TError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.message, self.location.to_string())
    }
}

impl std::error::Error for TError {}
