mod github;
mod telegram;
use anyhow::Result;


#[tokio::main]
async fn main() -> Result<()>{
    let personal_access_token = std::env::var("NOTEPAD_GITHUB_PERSONAL_ACCESS_TOKEN")?;
    let ghc = github::GithubClient::new(personal_access_token.to_string(), None);
    let gist_id = ghc.create_gist().await?;
    println!("{}", gist_id);

    Ok(())
}
