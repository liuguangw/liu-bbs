use super::commands::Commands;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct App {
    #[clap(subcommand)]
    command: Option<Commands>,
}

/// 命令行入口
pub fn run_app() {
    let app: App = App::parse();
    if let Some(sub_command) = &app.command {
        match sub_command {
            Commands::Hello(s) => s.execute(),
        }
        return;
    }
    println!("no sub command")
}
