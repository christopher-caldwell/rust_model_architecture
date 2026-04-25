#[derive(thiserror::Error, Debug)]
pub enum MemberError {
    #[error("Member not found")]
    NotFound,
    #[error("Member is already suspended")]
    CannotBeSuspended,
    #[error("Member is not currently suspended")]
    CannotBeReactivated,
    #[error("Member is suspended and cannot borrow new books")]
    CannotBorrowWhileSuspended,
    #[error("Member has reached the maximum number of active loans")]
    LoanLimitReached,
}
