use axum::{
    extract::{Path, State},
    Json,
};
use server_bootstrap::{MemberIdentInput, ServerDeps};

use crate::router::{
    auth::AuthUser,
    errors::{command_error, ApiError},
    members::schemas::{MemberResponseBody, MEMBERS_TAG},
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
    let input = MemberIdentInput {
        member_ident: ident,
    };
    let updated = deps.membership.commands.reactivate_member(input).await.map_err(command_error)?;
    Ok(Json(MemberResponseBody::from(updated)))
}
