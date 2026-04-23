use async_trait::async_trait;

use super::{BookCopy, BookCopyId, BookCopyPrepared, BookCopyStatus};

#[async_trait]
pub trait BookCopyWriteRepoPort: Send + Sync {
    async fn create(&self, insert: &BookCopyPrepared) -> anyhow::Result<BookCopy>;
    async fn get_by_barcode_for_update(&self, barcode: &str) -> anyhow::Result<Option<BookCopy>>;
    async fn update_status(
        &self,
        id: BookCopyId,
        status: BookCopyStatus,
    ) -> anyhow::Result<BookCopy>;
}

#[async_trait]
pub trait BookCopyReadRepoPort: Send + Sync {
    async fn get_by_id(&self, id: BookCopyId) -> anyhow::Result<Option<BookCopy>>;
    async fn get_by_barcode(&self, barcode: &str) -> anyhow::Result<Option<BookCopy>>;
}
