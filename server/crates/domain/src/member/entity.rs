use chrono::{DateTime, Utc};

pub struct Member {
    pub id: i16,
    pub ident: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub status: String,
    pub full_name: String,
    pub max_active_loans: i16

}

pub struct MemberCreationPayload {
    pub full_name: String,
    pub max_active_loans: i16
}

pub struct MemberPrepared {
    pub ident: String,
    pub full_name: String,
    pub max_active_loans: i16,
    pub status: &'static str
}

impl Member {
    #[must_use]
    pub fn can_be_suspended(&self) -> bool {
        return self.status != "suspended"
    }
    #[must_use]
    pub fn can_be_reactivated(&self) -> bool {
        return self.status == "suspended"
    }
    #[must_use]
    pub fn can_borrow(&self) -> bool {
        self.status == "active"
    }
    #[must_use]
    pub fn can_check_out_more_books(&self, active_loan_count: i64) -> bool {
        active_loan_count < i64::from(self.max_active_loans)
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
    // #[must_use]
    // pub fn is_spam(&self, likelihood: u8) -> bool {
    //     likelihood >= SPAM_LIKELIHOOD_THRESHOLD
    // }

    #[must_use]
    pub fn prepare(self, ident: String) -> MemberPrepared {
        // let is_spam = self.is_spam(spam_rating);
        let status: &str = "active";
        MemberPrepared {
            ident,
            full_name: self.full_name,
            max_active_loans: self.max_active_loans,
            status,
        }
    }
}
