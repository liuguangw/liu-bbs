use super::commands::Command;
use clap::Args;

/// 打印hello world的命令
#[derive(Args)]
pub struct HelloCommand;

impl Command for HelloCommand {
    fn execute(&self) {
        println!("hello world")
    }
}
