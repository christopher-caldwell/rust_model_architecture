use axum::{
    extract::{Path, State},
    Json,
};
use domain::member::MemberId;

use crate::router::{
    auth::AuthUser,
    dependencies::ServerDeps,
    errors::{not_found, service_error, ApiError},
    members::schemas::{MemberResponseBody, MEMBERS_TAG},
};

#[utoipa::path(
    delete,
    path = "/{id}/suspension",
    tag = MEMBERS_TAG,
    params(
        ("id" = i16, Path, description = "Identifier for the member")
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
    Path(id): Path<i16>,
) -> Result<Json<MemberResponseBody>, ApiError> {
    let member = match deps
        .membership
        .queries
        .get_member_details(MemberId(id))
        .await
    {
        Ok(Some(member)) => member,
        Ok(None) => return Err(not_found("Member not found")),
        Err(error) => return Err(service_error(error)),
    };

    deps.membership
        .commands
        .reactivate_member(member)
        .await
        .map(|updated| Json(MemberResponseBody::from(updated)))
        .map_err(service_error)
}
