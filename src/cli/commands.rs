use super::hello_command::HelloCommand;
use super::migrate_command::MigrateCommand;
use super::server_command::ServerCommand;
use clap::Subcommand;

///command定义
pub trait Command {
    ///执行command
    fn execute(&self);
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "hello", about = "hello world command")]
    Hello(HelloCommand),
    #[command(name = "serve", about = "run api server")]
    Serve(ServerCommand),
    #[command(name = "migrate", about = "data migration")]
    Migrate(MigrateCommand),
}

impl Command for Commands {
    fn execute(&self) {
        match self {
            Commands::Hello(s) => s.execute(),
            Commands::Serve(s) => s.execute(),
            Commands::Migrate(s) => s.execute(),
        }
    }
}
