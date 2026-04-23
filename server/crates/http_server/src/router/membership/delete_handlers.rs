use axum::{
    extract::{Path, State},
    Json,
};
use domain::member::MemberIdent;
use server_bootstrap::ServerDeps;

use crate::router::{
    auth::AuthUser,
    errors::{not_found, service_error, ApiError},
    membership::schemas::{MemberResponseBody, MEMBERS_TAG},
};

#[utoipa::path(
    delete,
    path = "/{ident}/suspension",
    tag = MEMBERS_TAG,
    params(
        ("ident" = String, Path, description = "Identifier for the member")
    ),
    responses(
        (status = 200, description = "Member reactivated", body = MemberResponseBody),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Member not found", body = crate::router::errors::ErrorResponseBody),
        (status = 409, description = "Member cannot be reactivated", body = crate::router::errors::ErrorResponseBody),
        (status = 500, description = "Internal server error", body = crate::router::errors::ErrorResponseBody)
    ),
    security(("bearer_auth" = []))
)]
pub async fn reactivate_member(
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

    let reactivate_member_result = deps.membership.commands.reactivate_member(member).await;

    let member_response = match reactivate_member_result {
        Ok(updated) => Json(MemberResponseBody::from(updated)),
        Err(error) => return Err(service_error(error)),
    };

    Ok(member_response)
}
