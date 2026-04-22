use async_trait::async_trait;

use crate::{
    book_copy::BookCopyId,
    member::{MemberId, MemberIdent},
};

use super::{Loan, LoanId, LoanPrepared};

#[async_trait]
pub trait LoanWriteRepoPort: Send + Sync {
    async fn create(&self, insert: &LoanPrepared) -> anyhow::Result<Loan>;
    async fn end(&self, id: LoanId) -> anyhow::Result<Loan>;
}

#[async_trait]
pub trait LoanReadRepoPort: Send + Sync {
    async fn get_by_member_ident(&self, ident: &MemberIdent) -> anyhow::Result<Vec<Loan>>;
    async fn get_overdue(&self) -> anyhow::Result<Vec<Loan>>;
    async fn find_active_by_book_copy_id(&self, id: BookCopyId) -> anyhow::Result<Option<Loan>>;
    async fn count_active_by_member_id(&self, id: MemberId) -> anyhow::Result<i64>;
}
