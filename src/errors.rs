use std::*;
use std::error::Error;
use std::process::Termination;

use log::error;

#[derive(Debug)]
pub enum WTTError {
    SetupLog(log::SetLoggerError),
    ConfigNotFound,
    XDG(xdg::BaseDirectoriesError),
}

pub type Result<T> = std::result::Result<T, WTTError>;

impl From<log::SetLoggerError> for WTTError {
    fn from(error: log::SetLoggerError) -> Self {
        WTTError::SetupLog(error)
    }
}

impl From<xdg::BaseDirectoriesError> for WTTError {
    fn from(error: xdg::BaseDirectoriesError) -> Self {
        WTTError::XDG(error)
    }
}

impl fmt::Display for WTTError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WTTError::SetupLog(_)    => write!(f, "Could not setup logger"),
            WTTError::ConfigNotFound => write!(f, "Config file could not be found"),
            WTTError::XDG(e)         => e.fmt(f),
        }
    }
}

impl Error for WTTError {}

impl Termination for WTTError {
    fn report(self) -> i32 {
        error!("{}", self);
        match self {
            WTTError::SetupLog(_)    => 10,
            WTTError::ConfigNotFound => 20,
            WTTError::XDG(_)         => 21,
        }
    }
}
