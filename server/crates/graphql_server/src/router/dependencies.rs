use application::commands::{CatalogCommands, LendingCommands, MembershipCommands};
use application::queries::{CatalogQueries, LendingQueries, MembershipQueries};
use std::sync::Arc;

#[derive(Clone)]
pub struct ServerDeps {
    pub auth: AuthDeps,
    pub catalog: CatalogDeps,
    pub lending: LendingDeps,
    pub membership: MembershipDeps,
}

#[derive(Clone)]
pub struct AuthDeps {
    pub jwt_secret: String,
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
