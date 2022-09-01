use super::app_command::AppCommand;
use super::commands::Commands;
use clap::Parser;

/// 命令行应用
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(arg_required_else_help(true))]
pub struct App {
    /// 子命令
    #[clap(subcommand)]
    command: Option<Commands>,
}

impl App {
    /// 命令行入口
    pub fn run() {
        let app: Self = Self::parse();
        if let Some(sub_command) = &app.command {
            sub_command.execute();
        }
    }
}
