use chrono::{DateTime, Utc};

use crate::{book_copy::BookCopyId, member::MemberId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LoanId(pub i64);

pub struct Loan {
    pub id: LoanId,
    pub ident: String,
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
    pub fn can_be_returned(&self) -> bool {
        self.dt_returned.is_none()
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
