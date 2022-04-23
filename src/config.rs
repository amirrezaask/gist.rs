use serde_derive::{Deserialize};
use anyhow::Result;

#[derive(Deserialize)]
pub struct Config {
    pub personal_access_token: String,
}

impl Config {
    pub async fn new(file_name: &str) -> Result<Self> {
        let config_file_content = tokio::fs::read_to_string(file_name).await?;
        let config: Config = toml::from_str(&config_file_content)?;
        Ok(config)
    }
}