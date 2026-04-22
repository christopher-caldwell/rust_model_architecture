pub mod get_handlers;
pub mod post_handlers;
pub mod schemas;

pub use get_handlers::get_overdue_loans;
pub use post_handlers::create_loan;
pub use schemas::{
    CreateLoanRequestBody, LoanResponseBody, LOANS_PATH, LOANS_TAG, OVERDUE_LOANS_PATH,
};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_handlers::get_overdue_loans, post_handlers::create_loan),
    components(schemas(
        CreateLoanRequestBody,
        LoanResponseBody,
        crate::router::errors::ErrorResponseBody
    ))
)]
pub struct LoansApi;
