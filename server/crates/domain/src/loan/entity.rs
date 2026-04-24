use chrono::{DateTime, Utc};

use crate::{book_copy::BookCopyId, member::MemberId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LoanId(pub i32);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LoanIdent(pub String);

impl From<LoanIdent> for String {
    fn from(value: LoanIdent) -> Self {
        value.0
    }
}

pub struct Loan {
    pub id: LoanId,
    pub ident: LoanIdent,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub book_copy_id: BookCopyId,
    pub member_id: MemberId,
    pub dt_due: Option<DateTime<Utc>>,
    pub dt_returned: Option<DateTime<Utc>>,
}

pub struct LoanCreationPayload {
    pub member_id: MemberId,
    pub book_copy_id: BookCopyId,
}

pub struct LoanPrepared {
    pub member_id: MemberId,
    pub book_copy_id: BookCopyId,
}

impl Loan {
    #[must_use]
    fn can_be_returned(&self) -> bool {
        self.dt_returned.is_none()
    }

    /// Guard: ensures loan has not already been returned.
    pub fn ensure_can_be_returned(&self) -> Result<(), LoanError> {
        if !self.can_be_returned() {
            return Err(LoanError::CannotBeReturned);
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LoanError {
    #[error("Book copy does not have an active loan")]
    NoActiveLoanForBookCopy,
    #[error("Loan has already been returned")]
    CannotBeReturned,
}

impl LoanCreationPayload {
    #[must_use]
    pub fn prepare(self) -> LoanPrepared {
        LoanPrepared {
            member_id: self.member_id,
            book_copy_id: self.book_copy_id,
        }
    }
}
