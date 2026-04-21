use async_trait::async_trait;
use domain::loan::Loan;

#[async_trait]
pub trait LoanReadRepoPort: Send + Sync {
    async fn find_active_by_book_copy_id(
        &self,
        book_copy_id: i64,
    ) -> anyhow::Result<Option<Loan>>;
    async fn count_active_by_member_id(
        &self,
        member_id: i64,
    ) -> anyhow::Result<i64>;
}
