use async_trait::async_trait;

use super::{Book, BookPrepared};

#[async_trait]
pub trait BookWriteRepoPort: Send + Sync {
    async fn create(&self, insert: &BookPrepared) -> anyhow::Result<Book>;
    async fn get_by_isbn(&self, isbn: &str) -> anyhow::Result<Option<Book>>;
}

#[async_trait]
pub trait BookReadRepoPort: Send + Sync {
    async fn get_catalog(&self) -> anyhow::Result<Vec<Book>>;
    async fn get_by_isbn(&self, isbn: &str) -> anyhow::Result<Option<Book>>;
}
