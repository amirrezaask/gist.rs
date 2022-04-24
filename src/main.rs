use clap::{Arg, ArgMatches, Command};

mod config;
use config::Config;

mod github;

fn parse_cli() -> ArgMatches {
    Command::new("gist")
        .author("amirrezaask, raskarpour@gmail.com")
        .version("0.1.0")
        .about("Github Gist CLI tool")
        .arg(Arg::new("config").required(false).short('c').long("config").takes_value(true).help("config file path"))
        .subcommand(
            Command::new("add").arg(
                Arg::new("file")
                    .required(true)
                    .short('f')
                    .long("file")
                    .takes_value(true)
                    .help("name of file to add to gist."),
            )
            .arg(Arg::new("desc").required(false).short('d').long("desc").takes_value(true).help("description of the gist")
        ))
        .get_matches()
}
async fn run_command(cfg: Config, arg_matches: ArgMatches) -> anyhow::Result<()> {
    let ghc = github::GithubClient::new(cfg.personal_access_token);
    match arg_matches.subcommand() {
        Some(("add", add_args)) => match add_args.value_of("file") {
            Some(file_name) => {
                let fd = tokio::fs::File::open(file_name).await?;
                let desc = add_args.value_of("desc");
                let gist_id = ghc.create_gist_from(file_name.to_string(), desc, fd).await?;
                println!("Link: {}", gist_id);
                Ok(())
            }
            _ => unreachable!(),
        },
        None => {
            println!("you should use a command");
            Ok(())
        }
        _ => unreachable!(),
    }
}
const DEFAULT_CONFIG_PATH: &str = "~/.config/gist/config.toml";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli_args = parse_cli();
    let config = Config::new(cli_args.value_of("config").unwrap_or(DEFAULT_CONFIG_PATH)).await?;
    run_command(config, cli_args).await;
    Ok(())
}
