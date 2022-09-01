use super::hello_command::HelloCommand;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    #[clap(name = "hello", about = "hello world command")]
    Hello(HelloCommand),
}
