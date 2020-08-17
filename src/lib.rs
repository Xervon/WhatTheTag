#![feature(termination_trait_lib)]
#![feature(try_trait)]

mod errors;
mod result;
mod config;

pub use errors::{ WTTError, Result };
pub use result::WTTResult;
pub use config::Config;
