use chrono::{DateTime, Utc};

pub struct Loan {
    pub id: i64,
    pub ident: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub book_copy_id: i64,
    pub member_id: i64,
    pub dt_due: Option<DateTime<Utc>>,
    pub dt_returned: Option<DateTime<Utc>>,
}

pub struct LoanCreationPayload {
    pub member_id: i64,
    pub book_copy: i64
}

pub struct LoanPrepared {
    pub member_id: i64,
    pub book_copy: i64
}

impl LoanCreationPayload {
    #[must_use]
    pub fn prepare(self) -> LoanPrepared {
        // Additional logic can be done here if needed
        LoanPrepared {
            member_id: self.member_id,
            book_copy: self.book_copy,
        }
    }
}
