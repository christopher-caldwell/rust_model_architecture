use axum::{extract::State, http::StatusCode, Json};

use crate::router::{
    auth::AuthUser,
    dependencies::ServerDeps,
    errors::{service_error, ApiError},
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
pub async fn create_member(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Json(body): Json<CreateMemberRequestBody>,
) -> Result<(StatusCode, Json<MemberResponseBody>), ApiError> {
    let register_member_result = deps.membership
        .commands
        .register_member(body.into())
        .await;

    let member_response = match register_member_result {
        Ok(member) => MemberResponseBody::from(member),
        Err(error) => return Err(service_error(error)),
    };

    Ok((
        StatusCode::CREATED,
        Json(member_response),
    ))
}
