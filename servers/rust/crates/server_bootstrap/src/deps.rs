use std::sync::Arc;

use anyhow::{Context, Result};
use application::ports::gen_ident::IdentGeneratorPort;
use auth_core::{AuthVerifierPort, JwtAuthAdapter};
use persistence::{
    book::BookReadRepoSql, book_copy::BookCopyReadRepoSql, loan::LoanReadRepoSql,
    member::MemberReadRepoSql, uow::SqlWriteUnitOfWorkFactory,
};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::config::ServerConfig;

pub use application::commands::{CatalogCommands, LendingCommands, MembershipCommands};
pub use application::queries::{CatalogQueries, LendingQueries, MembershipQueries};

#[derive(Clone)]
pub struct ServerDeps {
    pub auth: AuthDeps,
    pub catalog: CatalogDeps,
    pub lending: LendingDeps,
    pub membership: MembershipDeps,
}

#[derive(Clone)]
pub struct AuthDeps {
    pub verifier: Arc<dyn AuthVerifierPort + Send + Sync>,
}

#[derive(Clone)]
pub struct CatalogDeps {
    pub commands: Arc<CatalogCommands>,
    pub queries: Arc<CatalogQueries>,
}

#[derive(Clone)]
pub struct LendingDeps {
    pub commands: Arc<LendingCommands>,
    pub queries: Arc<LendingQueries>,
}

#[derive(Clone)]
pub struct MembershipDeps {
    pub commands: Arc<MembershipCommands>,
    pub queries: Arc<MembershipQueries>,
}

struct MemberIdentGenerator;

impl IdentGeneratorPort for MemberIdentGenerator {
    fn gen(&self) -> String {
        nanoid::nanoid!(10)
    }
}

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

    let book_read_repo = Arc::new(BookReadRepoSql {
        pool: ro_pool.clone(),
    });
    let book_copy_read_repo = Arc::new(BookCopyReadRepoSql {
        pool: ro_pool.clone(),
    });
    let loan_read_repo = Arc::new(LoanReadRepoSql {
        pool: ro_pool.clone(),
    });
    let member_read_repo = Arc::new(MemberReadRepoSql { pool: ro_pool });
    let ident_generator = Arc::new(MemberIdentGenerator);

    let catalog_commands = Arc::new(CatalogCommands::new(write_uow_factory.clone()));
    let lending_commands = Arc::new(LendingCommands::new(write_uow_factory.clone()));
    let membership_commands = Arc::new(MembershipCommands::new(write_uow_factory, ident_generator));

    let catalog_queries = Arc::new(CatalogQueries::new(book_read_repo, book_copy_read_repo));
    let lending_queries = Arc::new(LendingQueries::new(loan_read_repo));
    let membership_queries = Arc::new(MembershipQueries::new(member_read_repo));

    Ok(ServerDeps {
        auth: AuthDeps {
            verifier: Arc::new(JwtAuthAdapter::new(config.jwt_secret.clone())),
        },
        catalog: CatalogDeps {
            commands: catalog_commands,
            queries: catalog_queries,
        },
        lending: LendingDeps {
            commands: lending_commands,
            queries: lending_queries,
        },
        membership: MembershipDeps {
            commands: membership_commands,
            queries: membership_queries,
        },
    })
}
