use std::*;
use std::error::Error;
use std::process::Termination;

use log::error;

#[derive(Debug)]
pub enum WTTError {
    SetupLog(log::SetLoggerError),
}

pub type Result<T> = std::result::Result<T, WTTError>;

impl From<log::SetLoggerError> for WTTError {
    fn from(error: log::SetLoggerError) -> Self {
        WTTError::SetupLog(error)
    }
}

impl fmt::Display for WTTError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WTTError::SetupLog(_)    => write!(f, "Could not setup logger"),
        }
    }
}

impl Error for WTTError {}

impl Termination for WTTError {
    fn report(self) -> i32 {
        error!("{}", self);
        match self {
            WTTError::SetupLog(_)    => 10,
        }
    }
}
