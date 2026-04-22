use anyhow::{bail, Result};
use std::env;

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub database_ro_url: String,
    pub database_rw_url: String,
    pub jwt_secret: String,
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
/// Returns an error when required environment configuration is missing.
pub fn load_server_config() -> Result<ServerConfig> {
    Ok(ServerConfig {
        database_ro_url: required_env("DATABASE_RO_URL")?,
        database_rw_url: required_env("DATABASE_RW_URL")?,
        jwt_secret: required_env("JWT_SECRET")?,
    })
}
