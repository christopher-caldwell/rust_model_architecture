#[derive(thiserror::Error, Debug)]
pub enum BookCopyError {
    #[error("Book copy not found")]
    NotFound,
    #[error("Book cannot currently be borrowed")]
    CannotBeBorrowed,
    #[error("Book is not active and cannot be sent to maintenance")]
    CannotBeSentToMaintenance,
    #[error("Book is not currently in maintenance, and therefore cannot be returned")]
    CannotBeReturnedFromMaintenance,
    #[error("Book is already marked lost")]
    CannotMarkBookLost,
    #[error("Book is not currently lost, and cannot be returned from lost")]
    CannotBeReturnedFromLost,
}
