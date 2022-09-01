use clap::Args;

/// 打印hello world的命令
#[derive(Args)]
pub struct HelloCommand;

impl HelloCommand {
    pub fn execute(&self) {
        println!("hello world")
    }
}
