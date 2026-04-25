use anyhow::{bail, Context, Result};
use std::env;

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub database_ro_url: String,
    pub database_rw_url: String,
    pub jwt_secret: String,
    pub server_port: u16,
}

fn required_env(name: &str) -> Result<String> {
    let value = env::var(name).unwrap_or_default();

    if value.is_empty() {
        bail!("{name} must be set");
    }

    Ok(value)
}

/// # Errors
///
/// Returns an error when required environment configuration is missing or invalid.
pub fn load_server_config() -> Result<ServerConfig> {
    let server_port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .context("SERVER_PORT must be a valid port number (0-65535)")?;

    Ok(ServerConfig {
        database_ro_url: required_env("DATABASE_RO_URL")?,
        database_rw_url: required_env("DATABASE_RW_URL")?,
        jwt_secret: required_env("JWT_SECRET")?,
        server_port,
    })
}
