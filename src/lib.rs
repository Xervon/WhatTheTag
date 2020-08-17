#![feature(termination_trait_lib)]
#![feature(try_trait)]

mod errors;
mod result;

pub use errors::{ WTTError, Result };
pub use result::WTTResult;
