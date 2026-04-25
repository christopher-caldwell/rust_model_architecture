use axum::{extract::State, Json};
use server_bootstrap::ServerDeps;

use crate::router::{
    auth::AuthUser,
    errors::{service_error, ApiError},
    loan::schemas::{LoanResponseBody, LOANS_TAG},
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
    let loans = deps.lending.queries.get_overdue_loans().await.map_err(service_error)?;
    Ok(Json(loans.into_iter().map(LoanResponseBody::from).collect()))
}
