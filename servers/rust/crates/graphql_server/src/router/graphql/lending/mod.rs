use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use domain::loan::Loan;

pub mod mutations;
pub mod queries;

pub use mutations::LendingMutation;
pub use queries::LendingQuery;

#[derive(SimpleObject)]
pub struct LoanRecord {
    loan_number: String,
    dt_created: DateTime<Utc>,
    dt_modified: DateTime<Utc>,
    due_at: Option<DateTime<Utc>>,
    returned_at: Option<DateTime<Utc>>,
}

impl From<Loan> for LoanRecord {
    fn from(value: Loan) -> Self {
        Self {
            loan_number: value.ident.0,
            dt_created: value.dt_created,
            dt_modified: value.dt_modified,
            due_at: value.dt_due,
            returned_at: value.dt_returned,
        }
    }
}

#[derive(InputObject)]
pub struct StartLoanInput {
    member_number: String,
    barcode: String,
}
