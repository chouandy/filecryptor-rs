use std::error::Error;

use aws_types::region::Region;

pub async fn get_secrets_password_from_parameter_store(
    ps_region: &str,
    ps_name: &str,
) -> Result<String, Box<dyn Error>> {
    let region = Region::new(ps_region.to_string());
    let config = aws_config::from_env().region(region).load().await;
    let client = aws_sdk_ssm::Client::new(&config);
    let result = client
        .get_parameter()
        .name(ps_name)
        .with_decryption(true)
        .send()
        .await?;
    let password = result.parameter.unwrap().value.unwrap();

    Ok(password)
}
