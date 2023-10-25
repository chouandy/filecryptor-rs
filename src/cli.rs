use clap::Parser;

use crate::commands::Commands;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "filecryptor")]
#[command(about)]
#[command(version)]
#[command(subcommand_required = true)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub async fn run() {
        Self::parse().command.execute().await.unwrap();
    }
}
