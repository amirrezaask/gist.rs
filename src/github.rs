use anyhow::Result;
struct GithubClient {
    personal_access_token: String,
    gist_id: Option<String>,
}

impl GithubClient {
   pub async fn create_gist(&self) -> Result<String> {
        let app = octocrab::OctocrabBuilder::new().personal_token(self.personal_access_token.clone()).build()?;
        let gist = match &self.gist_id {
            Some(gist_id) => app.gists().get(gist_id).await?, 
            None => app.gists().create().public(false).description("my github gist").file("notes", "").send().await?,
        };
        Ok(gist.id)
   }
}