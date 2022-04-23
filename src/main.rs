mod github;
mod telegram;
use anyhow::Result;


#[tokio::main]
async fn main() -> Result<()>{
    let ghc = github::GithubClient::new(PERSONAL_ACCESS_TOKEN.to_string(), None);
    let gist_id = ghc.create_gist().await?;
    println!("{}", gist_id);

    Ok(())
}
