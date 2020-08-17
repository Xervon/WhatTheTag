use std::path::PathBuf;
use xdg::BaseDirectories;

use crate::{ Result, WTTError };

pub struct Config {
}

impl Config {
    pub fn get_config_location() -> Result<PathBuf> {
        let base_dirs = BaseDirectories::with_prefix("wtt")?;

        match base_dirs.find_config_file("config.json") {
            Some(p) => Result::Ok(p),
            None    => Result::Err(WTTError::ConfigNotFound),
        }
    }
}
