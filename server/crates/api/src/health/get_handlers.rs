use axum::{extract::State, http::StatusCode, Json};

use crate::{
    dependencies::ServerDeps,
    health::schemas::{HealthCheckResponseBody, HEALTH_CHECK_TAG},
};

#[utoipa::path(
    get,
    path = "",
    tag = HEALTH_CHECK_TAG,
    responses(
        (status = 200, description = "Server ready to accept requests", body = HealthCheckResponseBody),
        (status = 500, description = "Internal server error")
    ),
)]
/// Performs a health check.
///
/// # Errors
///
/// This function is currently infallible but returns a `Result` for consistency with other handlers.
pub async fn get_health_check(
    State(_deps): State<ServerDeps>,
) -> Result<Json<HealthCheckResponseBody>, StatusCode> {
    let response = HealthCheckResponseBody {
        message: String::from("ready"),
    };
    Ok(Json(response))
}

// ------------------------------------------------------------------------------------------------------------------------------//
// ------------------------------------------------------------------------------------------------------------------------------//
