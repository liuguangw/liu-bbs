use super::commands::{Command, Commands};
use crate::common;
use clap::Parser;

/// 命令行应用
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true), disable_version_flag(true))]
pub struct App {
    /// 子命令
    #[command(subcommand)]
    command: Option<Commands>,
    ///Print version info and exit
    #[arg(short = 'V', long = "version", value_parser)]
    show_version: bool,
    ///Use verbose output
    #[arg(short, long, value_parser)]
    verbose: bool,
}

impl App {
    /// 命令行入口
    pub fn run() {
        let app: Self = Self::parse();
        if let Some(sub_command) = &app.command {
            sub_command.execute();
            return;
        }
        if app.show_version {
            let version_string = common::get_version_string(app.verbose);
            println!("{version_string}");
        }
    }
}
