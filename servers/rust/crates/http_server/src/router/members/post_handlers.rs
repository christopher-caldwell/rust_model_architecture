use axum::{extract::State, http::StatusCode, Json};
use server_bootstrap::ServerDeps;

use crate::router::{
    auth::AuthUser,
    errors::{command_error, ApiError},
    members::schemas::{CreateMemberRequestBody, MemberResponseBody, MEMBERS_TAG},
};

#[utoipa::path(
    post,
    path = "",
    tag = MEMBERS_TAG,
    request_body = CreateMemberRequestBody,
    responses(
        (status = 201, description = "Member created", body = MemberResponseBody),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error", body = crate::router::errors::ErrorResponseBody)
    ),
    security(("bearer_auth" = []))
)]
pub async fn register_member(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Json(body): Json<CreateMemberRequestBody>,
) -> Result<(StatusCode, Json<MemberResponseBody>), ApiError> {
    let member = deps.membership.commands.register_member(body.into()).await.map_err(command_error)?;
    Ok((StatusCode::CREATED, Json(MemberResponseBody::from(member))))
}
