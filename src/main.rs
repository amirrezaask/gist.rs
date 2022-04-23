use clap::{Command, ArgMatches};

mod config;
use config::Config;

mod github;

fn get_cli_args() -> ArgMatches {
    Command::new("gist")
        .author("amirrezaask, raskarpour@gmail.com")
        .version("0.1.0")
        .about("Github Gist CLI tool")
        .arg(clap::arg!([file] -f --file "file to add as gist"))
        .arg(clap::arg!([gist_id] -g --gist_id "optional gist id to add file into if not provided creates a new gist."))
        .get_matches()
}

#[tokio::main]
 async fn main() -> anyhow::Result<()> {
    let cli_args = get_cli_args();
    let config = Config::new("config.toml").await?;
    let ghc = github::GithubClient::new(config.personal_access_token);
    let file_name = match cli_args.value_of("file") {
        Some(f) => f,
        None => return Err(anyhow::anyhow!("no file provided")) 
    };

    let fd = tokio::fs::File::open(file_name).await?;
    let gist_id = ghc.create_gist_from(file_name.to_string(), fd).await?;
    println!("{}", gist_id);
    Ok(())

}


// #[derive(Debug)]
// pub enum Errors {
//     NoFileProvided,
// }

// impl std::fmt::Display for Errors {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             NoFileProvided => f.write_str("No file provided to add to gist"), 
//         }
//     }
// }

// impl std::error::Error for Errors {}


