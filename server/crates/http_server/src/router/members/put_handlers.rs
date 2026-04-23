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
    put,
    path = "/{ident}/suspension",
    tag = MEMBERS_TAG,
    params(
        ("ident" = String, Path, description = "Identifier for the member")
    ),
    responses(
        (status = 200, description = "Member suspended", body = MemberResponseBody),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Member not found", body = crate::router::errors::ErrorResponseBody),
        (status = 409, description = "Member cannot be suspended", body = crate::router::errors::ErrorResponseBody),
        (status = 500, description = "Internal server error", body = crate::router::errors::ErrorResponseBody)
    ),
    security(("bearer_auth" = []))
)]
pub async fn suspend_member(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Path(ident): Path<String>,
) -> Result<Json<MemberResponseBody>, ApiError> {
    let input = MemberIdentInput {
        member_ident: ident,
    };
    let suspend_member_result = deps.membership.commands.suspend_member(input).await;

    let member_response = match suspend_member_result {
        Ok(updated) => Json(MemberResponseBody::from(updated)),
        Err(error) => return Err(command_error(error)),
    };

    Ok(member_response)
}
