use async_trait::async_trait;
use domain::{book::Book, book_copy::BookCopy, loan::Loan, member::Member};

#[async_trait]
pub trait CatalogReadRepoPort: Send + Sync {
    async fn get_book_catalog(
        &self,
    ) -> anyhow::Result<Vec<Book>>;
    async fn get_book_copy_details(
        &self,
        book_copy_id: i64,
    ) -> anyhow::Result<Option<BookCopy>>;
}

#[async_trait]
pub trait MemberReadRepoPort: Send + Sync {
    async fn get_member_details(
        &self,
        member_id: i16,
    ) -> anyhow::Result<Option<Member>>;
}

#[async_trait]
pub trait LoanReadRepoPort: Send + Sync {
    async fn get_member_loans(
        &self,
        member_id: i64,
    ) -> anyhow::Result<Vec<Loan>>;
    async fn get_overdue_loans(
        &self,
    ) -> anyhow::Result<Vec<Loan>>;
    async fn find_active_by_book_copy_id(
        &self,
        book_copy_id: i64,
    ) -> anyhow::Result<Option<Loan>>;
    async fn count_active_by_member_id(
        &self,
        member_id: i64,
    ) -> anyhow::Result<i64>;
}
