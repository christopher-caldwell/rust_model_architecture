use chrono::{DateTime, Utc};
use domain::loan::Loan;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const LOANS_TAG: &str = "lending";
pub const LOANS_PATH: &str = "/loans";
pub const OVERDUE_LOANS_PATH: &str = "/loans/overdue";

#[derive(Serialize, ToSchema)]
pub struct LoanResponseBody {
    pub ident: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub dt_due: Option<DateTime<Utc>>,
    pub dt_returned: Option<DateTime<Utc>>,
}

impl From<Loan> for LoanResponseBody {
    fn from(value: Loan) -> Self {
        Self {
            ident: value.ident.into(),
            dt_created: value.dt_created,
            dt_modified: value.dt_modified,
            dt_due: value.dt_due,
            dt_returned: value.dt_returned,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateLoanRequestBody {
    pub member_ident: String,
    pub book_copy_barcode: String,
}
