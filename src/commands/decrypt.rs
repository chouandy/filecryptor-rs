use std::error::Error;

use clap::Args;

use crate::{filecrypt::decrypt_file, secrets::get_secrets_password_from_parameter_store};

#[derive(Debug, Args)]
pub struct Command {
    /// Target file to be decrypted.
    #[arg(long, short, required = true)]
    pub file: String,

    /// The password for decrypt
    #[arg(
        long,
        env = "SECRETS_PASSWORD",
        conflicts_with = "ps_name",
        conflicts_with = "ps_region",
        required_unless_present = "ps_name",
        required_unless_present = "ps_region"
    )]
    pub password: Option<String>,

    /// The parameter store name for decrypt
    #[arg(
        long,
        env = "SECRETS_PASSWORD_PS_NAME",
        conflicts_with = "password",
        requires = "ps_region"
    )]
    pub ps_name: Option<String>,

    /// The parameter store region for decrypt
    #[arg(
        long,
        env = "SECRETS_PASSWORD_PS_REGION",
        conflicts_with = "password",
        requires = "ps_name"
    )]
    pub ps_region: Option<String>,
}

impl Command {
    pub async fn execute(&self) -> Result<(), Box<dyn Error>> {
        let password = match &self.password {
            Some(password) => password.to_string(),
            None => match (&self.ps_region, &self.ps_name) {
                (Some(region), Some(name)) => {
                    print!("Get password from Parameter Store...");
                    let password = get_secrets_password_from_parameter_store(region, name).await?;
                    println!("done");
                    password
                }
                _ => return Err("Password not provided".into()),
            },
        };

        let src = self.file.to_string();
        let dst = src.trim_end_matches(".enc").to_string();

        print!("Decrypting file...");
        decrypt_file(&src, &dst, password.as_bytes())?;
        println!("done");

        Ok(())
    }
}
