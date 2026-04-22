use chrono::{DateTime, Utc};
use domain::loan::Loan;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const LOANS_TAG: &str = "Loans";
pub const LOANS_PATH: &str = "/loans";
pub const OVERDUE_LOANS_PATH: &str = "/loans/overdue";

#[derive(Serialize, ToSchema)]
pub struct LoanResponseBody {
    pub id: i64,
    pub ident: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub book_copy_id: i64,
    pub member_id: i16,
    pub dt_due: Option<DateTime<Utc>>,
    pub dt_returned: Option<DateTime<Utc>>,
}

impl From<Loan> for LoanResponseBody {
    fn from(value: Loan) -> Self {
        Self {
            id: value.id.0,
            ident: value.ident,
            dt_created: value.dt_created,
            dt_modified: value.dt_modified,
            book_copy_id: value.book_copy_id.0,
            member_id: value.member_id.0,
            dt_due: value.dt_due,
            dt_returned: value.dt_returned,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateLoanRequestBody {
    pub member_id: i16,
    pub book_copy_id: i64,
}
