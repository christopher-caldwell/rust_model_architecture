use axum::{
    extract::{Path, State},
    Json,
};
use domain::member::MemberId;

use crate::router::{
    auth::AuthUser,
    dependencies::ServerDeps,
    errors::{not_found, service_error, ApiError},
    loans::schemas::LoanResponseBody,
    members::schemas::{MemberResponseBody, MEMBERS_TAG},
};

#[utoipa::path(
    get,
    path = "/{id}",
    tag = MEMBERS_TAG,
    params(
        ("id" = i16, Path, description = "Identifier for the member")
    ),
    responses(
        (status = 200, description = "Member details", body = MemberResponseBody),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Member not found", body = crate::router::errors::ErrorResponseBody),
        (status = 500, description = "Internal server error", body = crate::router::errors::ErrorResponseBody)
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_member_by_id(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Path(id): Path<i16>,
) -> Result<Json<MemberResponseBody>, ApiError> {
    match deps
        .membership
        .queries
        .get_member_details(MemberId(id))
        .await
    {
        Ok(Some(member)) => Ok(Json(MemberResponseBody::from(member))),
        Ok(None) => Err(not_found("Member not found")),
        Err(error) => Err(service_error(error)),
    }
}

#[utoipa::path(
    get,
    path = "/{id}/loans",
    tag = MEMBERS_TAG,
    params(
        ("id" = i16, Path, description = "Identifier for the member")
    ),
    responses(
        (status = 200, description = "Loans for a member", body = Vec<LoanResponseBody>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error", body = crate::router::errors::ErrorResponseBody)
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_member_loans(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Path(id): Path<i16>,
) -> Result<Json<Vec<LoanResponseBody>>, ApiError> {
    deps.lending
        .queries
        .get_member_loans(MemberId(id))
        .await
        .map(|loans| Json(loans.into_iter().map(LoanResponseBody::from).collect()))
        .map_err(service_error)
}
