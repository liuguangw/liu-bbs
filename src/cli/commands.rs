use super::app_command::AppCommand;
use super::hello_command::HelloCommand;
use super::migrate_command::MigrateCommand;
use super::server_command::ServerCommand;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    #[clap(name = "hello", about = "hello world command")]
    Hello(HelloCommand),
    #[clap(name = "serve", about = "run api server")]
    Serve(ServerCommand),
    #[clap(name = "migrate", about = "data migration")]
    Migrate(MigrateCommand),
}

impl AppCommand for Commands {
    fn execute(&self) {
        match self {
            Commands::Hello(s) => s.execute(),
            Commands::Serve(s) => s.execute(),
            Commands::Migrate(s) => s.execute(),
        }
    }
}
