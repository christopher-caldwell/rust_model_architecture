use async_trait::async_trait;

use crate::ports::write_repos::{BookWriteRepoPort, MemberWriteRepoPort, LoanWriteRepoPort, BookCopyWriteRepoPort};


#[async_trait]
pub trait UnitOfWorkPort: Send + Sync {
    fn book_write_repo(&self) -> &dyn BookWriteRepoPort;
    fn book_copy_write_repo(&self) -> &dyn BookCopyWriteRepoPort;
    fn membership_write_repo(&self) -> &dyn MemberWriteRepoPort;
    fn loan_write_repo(&self) -> &dyn LoanWriteRepoPort;
    async fn commit(self: Box<Self>) -> anyhow::Result<()>;
}

#[async_trait]
pub trait WriteUnitOfWorkFactory: Send + Sync {
    async fn build(&self) -> anyhow::Result<Box<dyn UnitOfWorkPort>>;
}
