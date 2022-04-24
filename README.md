# Gist Cli 
Simple CLI tool to manage your github gist.

## Usage 
First create a configuration file in "$HOME/.config/gist/config.toml":
```toml
personal_access_token = '<Your Personal Access Token>'
```
*Note*: It can be in other places but then you need to pass the path through --config/-c flag.
then if you invoke gist command:
```bash
gist 0.1.0
amirrezaask, raskarpour@gmail.com
Github Gist CLI tool

USAGE:
    gist [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -c, --config <config>    config file path
    -h, --help               Print help information
    -V, --version            Print version information

SUBCOMMANDS:
    add
    help    Print this message or the help of the given subcommand(s)
```
you can use each command and see their help using --help.
### Add command
```bash
USAGE:
    gist add [OPTIONS] --file <file>

OPTIONS:
    -d, --desc <desc>    description of the gist
    -f, --file <file>    name of file to add to gist.
    -h, --help           Print help information
```