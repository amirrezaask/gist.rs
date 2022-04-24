# Gist Cli 
Simple CLI tool to manage your github gist.

## Usage 
First create a configuration file in "$HOME/.config/gist/config.toml":
```toml
personal_access_token = '<Your Personal Access Token>'
```
*Note*: It can be in other places but then you need to pass the path through --config/-c flag.
### Add command
```bash
USAGE:
    gist add [OPTIONS] --file <file>

OPTIONS:
    -d, --desc <desc>    description of the gist
    -f, --file <file>    name of file to add to gist.
    -h, --help           Print help information
```