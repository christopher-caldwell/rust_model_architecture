use axum::{extract::State, Json};
use server_bootstrap::ServerDeps;

use crate::router::{
    auth::AuthUser,
    errors::{service_error, ApiError},
    lending::schemas::{LoanResponseBody, LOANS_TAG},
};

#[utoipa::path(
    get,
    path = "/overdue",
    tag = LOANS_TAG,
    responses(
        (status = 200, description = "Overdue loans", body = Vec<LoanResponseBody>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error", body = crate::router::errors::ErrorResponseBody)
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_overdue_loans(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
) -> Result<Json<Vec<LoanResponseBody>>, ApiError> {
    let overdue_loans_result = deps.lending.queries.get_overdue_loans().await;

    let overdue_loans = match overdue_loans_result {
        Ok(loans) => loans,
        Err(error) => return Err(service_error(error)),
    };

    let overdue_loan_response = overdue_loans
        .into_iter()
        .map(LoanResponseBody::from)
        .collect();

    Ok(Json(overdue_loan_response))
}
