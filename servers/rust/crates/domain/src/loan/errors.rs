#[derive(thiserror::Error, Debug)]
pub enum LoanError {
    #[error("Book copy does not have an active loan")]
    NoActiveLoanForBookCopy,
    #[error("Loan has already been returned")]
    CannotBeReturned,
}
