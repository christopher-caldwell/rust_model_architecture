use axum::{extract::State, Json};

use crate::router::{
    auth::AuthUser,
    dependencies::ServerDeps,
    errors::{service_error, ApiError},
    loans::schemas::{LoanResponseBody, LOANS_TAG},
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
    deps.lending
        .queries
        .get_overdue_loans()
        .await
        .map(|loans| Json(loans.into_iter().map(LoanResponseBody::from).collect()))
        .map_err(service_error)
}
