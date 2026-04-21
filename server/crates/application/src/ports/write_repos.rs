use async_trait::async_trait;
use domain::{book::{BookPrepared, Book}, book_copy::{BookCopyPrepared, BookCopy}, loan::{Loan, LoanPrepared}, member::{Member, MemberPrepared}};

#[async_trait]
pub trait BookWriteRepoPort: Send + Sync {
    async fn create(
        &self,
        insert: &BookPrepared,
    ) -> Result<Book, String>;
}

#[async_trait]
pub trait BookCopyWriteRepoPort: Send + Sync {
    async fn create(
        &self,
        insert: &BookCopyPrepared,
    ) -> Result<BookCopy, String>;
    async fn update_status(
        &self,
        book_copy_id: i64,
        status: &str
    ) -> Result<BookCopy, String>;
}

#[async_trait]
pub trait MemberWriteRepoPort: Send + Sync {
    async fn create(
        &self,
        insert: &MemberPrepared,
    ) -> Result<Member, String>;
}

#[async_trait]
pub trait LoanWriteRepoPort: Send + Sync {
    async fn create(
        &self,
        insert: &LoanPrepared,
    ) -> Result<Loan, String>;
    async fn end(
        &self,
        loan_id: i64,
    ) -> Result<Loan, String>;
}

// #[async_trait]
// pub trait ContactInquirySpamRatingPort: Send + Sync {
//     async fn get_spam_likelihood(&self, message: &str) -> Result<u8, String>;
// }
