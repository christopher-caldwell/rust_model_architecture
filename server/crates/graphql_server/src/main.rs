use anyhow::{anyhow, Context, Result};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

use http_server::{config, deps, router::new_router};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init()
        .map_err(|error| anyhow!(error.to_string()))?;

    dotenv().ok();
    let config = config::load_server_config()?;
    let deps = deps::create_server_deps(&config).await?;
    let app = new_router(deps);
    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(address)
        .await
        .with_context(|| format!("failed to bind HTTP listener on {address}"))?;

    info!("CRM server listening on :3000");

    axum::serve(listener, app)
        .await
        .context("crm server exited unexpectedly")?;

    Ok(())
}
