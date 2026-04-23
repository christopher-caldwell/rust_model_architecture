use axum::{
    extract::{Path, State},
    Json,
};
use domain::member::MemberIdent;
use server_bootstrap::ServerDeps;

use crate::router::{
    auth::AuthUser,
    errors::{not_found, service_error, ApiError},
    lending::schemas::LoanResponseBody,
    membership::schemas::{MemberResponseBody, MEMBERS_TAG},
};

#[utoipa::path(
    get,
    path = "/{ident}",
    tag = MEMBERS_TAG,
    params(
        ("ident" = String, Path, description = "Identifier for the member")
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
pub async fn get_member_details(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Path(ident): Path<String>,
) -> Result<Json<MemberResponseBody>, ApiError> {
    let member_result = deps
        .membership
        .queries
        .get_member_details(&MemberIdent(ident))
        .await;

    let member = match member_result {
        Ok(Some(member)) => member,
        Ok(None) => return Err(not_found("Member not found")),
        Err(error) => return Err(service_error(error)),
    };

    Ok(Json(MemberResponseBody::from(member)))
}

#[utoipa::path(
    get,
    path = "/{ident}/loans",
    tag = MEMBERS_TAG,
    params(
        ("ident" = String, Path, description = "Identifier for the member")
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
    Path(ident): Path<String>,
) -> Result<Json<Vec<LoanResponseBody>>, ApiError> {
    let member_loans_result = deps
        .lending
        .queries
        .get_member_loans(&MemberIdent(ident))
        .await;

    let member_loans = match member_loans_result {
        Ok(loans) => loans,
        Err(error) => return Err(service_error(error)),
    };

    let member_loans_response = member_loans
        .into_iter()
        .map(LoanResponseBody::from)
        .collect();

    Ok(Json(member_loans_response))
}
