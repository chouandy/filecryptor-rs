mod decrypt;
mod encrypt;
mod hex;

use std::error::Error;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Encrypt file
    #[command(arg_required_else_help = true)]
    Enc(encrypt::Command),

    /// Decrypt file
    #[command(arg_required_else_help = true)]
    Dec(decrypt::Command),

    /// Random hex string
    #[command(arg_required_else_help = true)]
    Hex(hex::Command),
}

impl Commands {
    pub async fn execute(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Commands::Enc(command) => command.execute().await,
            Commands::Dec(command) => command.execute().await,
            Commands::Hex(command) => command.execute().await,
        }
    }
}
