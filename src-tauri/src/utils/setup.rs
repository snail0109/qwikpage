use crate::utils::dirs;
use anyhow::Result;
use std::{fs, io::Error};

pub fn init() -> Result<(), Error> {
    let app_config_dir = dirs::app_config_dir().unwrap();
    let app_data_dir = dirs::app_data_dir().unwrap();
    if !app_config_dir.exists() {
        fs::create_dir_all(&app_config_dir)
            .map_err(|e| Error::new(std::io::ErrorKind::InvalidInput, e))?;
    }
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| Error::new(std::io::ErrorKind::InvalidInput, e))?;
    }
    Ok(())
}
