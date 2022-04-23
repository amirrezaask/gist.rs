use anyhow::Result;
use tokio::io::{AsyncRead, AsyncReadExt};

pub struct GithubClient {
    personal_access_token: String,
}

impl GithubClient {
    pub fn new(personal_access_token: String) -> Self {
        Self {
            personal_access_token, 
        }

    }
    pub async fn create_gist_from(&self, name: String, mut file: impl AsyncRead + Unpin) -> Result<String> {
        let app = octocrab::OctocrabBuilder::new()
            .personal_token(self.personal_access_token.clone())
            .build()?;

        let mut content = String::new();
        file.read_to_string(&mut content).await?;

        let gist = app.gists()
            .create()
            .public(false)
            .description("Gist created using Gist tool")
            .file(name, content)
            .send()
            .await?;

        Ok(gist.id)
    }
}
