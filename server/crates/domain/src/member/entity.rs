use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MemberId(pub i32);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MemberIdent(pub String);

impl From<MemberIdent> for String {
    fn from(value: MemberIdent) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemberStatus {
    Active,
    Suspended,
}

pub struct Member {
    pub id: MemberId,
    pub ident: MemberIdent,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub status: MemberStatus,
    pub full_name: String,
    pub max_active_loans: i16,
}

pub struct MemberCreationPayload {
    pub full_name: String,
    pub max_active_loans: i16,
}

pub struct MemberPrepared {
    pub ident: MemberIdent,
    pub full_name: String,
    pub max_active_loans: i16,
    pub status: MemberStatus,
}

impl Member {
    #[must_use]
    pub fn can_be_suspended(&self) -> bool {
        self.status != MemberStatus::Suspended
    }

    #[must_use]
    pub fn can_be_reactivated(&self) -> bool {
        self.status == MemberStatus::Suspended
    }

    #[must_use]
    pub fn can_borrow(&self) -> bool {
        self.status == MemberStatus::Active
    }

    #[must_use]
    pub fn can_check_out_more_books(&self, active_loan_count: i16) -> bool {
        active_loan_count < self.max_active_loans
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MemberError {
    #[error("Member is already suspended")]
    CannotBeSuspended,
    #[error("Member is not currently suspended")]
    CannotBeReactivated,
    #[error("Member is suspended and cannot borrow new books")]
    CannotBorrowWhileSuspended,
    #[error("Member has reached the maximum number of active loans")]
    LoanLimitReached,
}

impl MemberCreationPayload {
    #[must_use]
    pub fn prepare(self, ident: MemberIdent) -> MemberPrepared {
        MemberPrepared {
            ident,
            full_name: self.full_name,
            max_active_loans: self.max_active_loans,
            status: MemberStatus::Active,
        }
    }
}
