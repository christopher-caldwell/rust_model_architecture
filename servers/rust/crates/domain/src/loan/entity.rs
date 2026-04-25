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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct LoanCreationPayload {
    pub member_id: MemberId,
    pub book_copy_id: BookCopyId,
}

#[derive(Debug, Clone)]
pub struct LoanPrepared {
    pub member_id: MemberId,
    pub book_copy_id: BookCopyId,
}
