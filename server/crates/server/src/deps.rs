use std::{collections::HashMap, sync::Arc};

use anyhow::{Context, Result};
use api::dependencies::{AuthDeps, ContactInquiryDeps, ServerDeps};
use application::{ContactInquiryCommands, ContactInquiryQueries};
use infrastructure::{
    llm::ContactInquirySpamRatingAdapter,
    llm_provider::anthropic::{AnthropicClient, AnthropicModel},
};
use persistence::{
    contact_inquiry::{ContactInquiryReadRepoSql, ContactInquiryStatusRepoSql},
    uow::SqlWriteUnitOfWorkFactory,
};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::config::ServerConfig;

async fn connect_pool(database_url: &str, label: &str) -> Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .with_context(|| format!("failed to connect to {label} database"))
}

/// # Errors
///
/// Returns an error when any dependency cannot be initialized.
#[allow(clippy::similar_names)]
pub async fn create_server_deps(config: &ServerConfig) -> Result<ServerDeps> {
    let ro_pool = connect_pool(&config.database_ro_url, "read-only").await?;
    let rw_pool = connect_pool(&config.database_rw_url, "read-write").await?;

    let write_uow_factory = Arc::new(SqlWriteUnitOfWorkFactory { pool: rw_pool });
    let contact_inquiry_read = Arc::new(ContactInquiryReadRepoSql {
        pool: ro_pool.clone(),
    });
    let contact_inquiry_status = Arc::new(ContactInquiryStatusRepoSql {
        pool: ro_pool,
        ident_cache: tokio::sync::RwLock::new(HashMap::new()),
        id_cache: tokio::sync::RwLock::new(HashMap::new()),
    });
    let contact_inquiry_spam = Arc::new(ContactInquirySpamRatingAdapter {
        llm_client: AnthropicClient {
            api_key: config.anthropic_api_key.clone(),
            model: AnthropicModel::Claude4_5Haiku,
            http_client: reqwest::Client::new(),
        },
    });

    let contact_inquiry_commands = Arc::new(ContactInquiryCommands::new(
        write_uow_factory,
        contact_inquiry_spam,
    ));
    let contact_inquiry_queries = Arc::new(ContactInquiryQueries::new(
        contact_inquiry_read,
        contact_inquiry_status,
    ));

    Ok(ServerDeps {
        auth: AuthDeps {
            jwt_secret: config.jwt_secret.clone(),
        },
        contact_inquiry: ContactInquiryDeps {
            commands: contact_inquiry_commands,
            queries: contact_inquiry_queries,
        },
    })
}
