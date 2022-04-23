mod github;
mod telegram;
use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use toml::Value;

#[derive(Deserialize)]
struct Config {
    pub personal_access_token: String,
    pub gist_id: String,
}

impl Config {
    async fn new(file_name: &str) -> Result<Self> {
        let config_file_content = tokio::fs::read_to_string(file_name).await?;
        let config: Config = toml::from_str(&config_file_content)?;
        Ok(config)
    }
}


#[tokio::main]
async fn main() -> Result<()>{
    let config = Config::new("config.toml").await?;
    let ghc = github::GithubClient::new(config.personal_access_token, None);
    let gist_id = ghc.create_gist().await?;
    println!("{}", gist_id);
    Ok(())
}
