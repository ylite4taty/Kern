use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(debug, Serialize, Deserialize)]
pub struct Config {
    pub hotkey: String,
    pub docs_path: String,
    pub index_path: String,
}

impl Default for Config {
    fn default() -> Self {
       Self {
           hotkey: "super+k".to_string(),
           docs_path: "./docs".to_string(),
           index_path "./index".to_string(),
       }
    }

}

pub fn load() -> Result<Config> {
    let path = "config/default.toml";

    if !std::path::Path::new(path).exists(){
        return Ok(Config::default());

    }


    let contents = fs::read_to_string(path)?
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

