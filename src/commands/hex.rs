use std::error::Error;

use clap::Args;
use rand_core::{OsRng, RngCore};

#[derive(Debug, Args)]
pub struct Command {
    /// The size of hex
    #[arg(long, short, required = true)]
    pub size: usize,
}

impl Command {
    pub async fn execute(&self) -> Result<(), Box<dyn Error>> {
        println!("{}", generate_hex_text(self.size));

        Ok(())
    }
}

fn generate_hex_text(size: usize) -> String {
    let mut hex_bytes = vec![0u8; size];
    OsRng.fill_bytes(&mut hex_bytes);
    hex::encode(hex_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_hex_text() {
        let size = 10;
        let hex_text = generate_hex_text(size);
        assert_eq!(hex_text.len(), size * 2);
        assert!(hex_text.chars().all(|c| "0123456789abcdef".contains(c)));
    }
}
