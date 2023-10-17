use std::{fs::read_to_string, path::PathBuf};

#[derive(Debug, serde::Deserialize)]
pub struct RdropConfig {
    pub connection_string: String,
    pub jobs_to_run: Option<Vec<String>>,
}

impl RdropConfig {
    pub fn new(at: PathBuf) -> Result<Self, String> {
        if !at.exists() {
            return Err(format!(
                "Could not locate a config file at {}",
                at.to_str().unwrap()
            ));
        }

        let read = match read_to_string(&at) {
            Ok(str) => str,
            Err(_) => {
                return Err(format!(
                "Could not read file at {}. Perhaps I don't have the rights to read that file...",
                at.to_str().unwrap()
            ))
            }
        };

        let toml: Result<Self, toml::de::Error> = toml::from_str(&read);

        match toml {
            Ok(toml) => Ok(toml),
            Err(err) => Err(err.to_string()),
        }
    }
}
