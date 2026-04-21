use async_trait::async_trait;

use crate::contact_inquiry::ContactInquiryWriteRepoPort;

#[async_trait]
pub trait UnitOfWorkPort: Send + Sync {
    fn contact_inquiry_write_repo(&self) -> &dyn ContactInquiryWriteRepoPort;
    async fn commit(self: Box<Self>) -> anyhow::Result<()>;
}

#[async_trait]
pub trait WriteUnitOfWorkFactory: Send + Sync {
    async fn build(&self) -> anyhow::Result<Box<dyn UnitOfWorkPort>>;
}
