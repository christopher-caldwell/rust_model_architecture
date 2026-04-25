use domain::{book::BookError, book_copy::BookCopyError, loan::LoanError, member::MemberError};

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error(transparent)]
    Member(#[from] MemberError),

    #[error(transparent)]
    BookCopy(#[from] BookCopyError),

    #[error(transparent)]
    Book(#[from] BookError),

    #[error(transparent)]
    Loan(#[from] LoanError),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}
