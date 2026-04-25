use axum::{extract::State, Json};
use server_bootstrap::ServerDeps;

use crate::router::{
    auth::AuthUser,
    books::schemas::{BookResponseBody, BOOKS_TAG},
    errors::{service_error, ApiError},
};

#[utoipa::path(
    get,
    path = "",
    tag = BOOKS_TAG,
    responses(
        (status = 200, description = "Book catalog", body = Vec<BookResponseBody>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error", body = crate::router::errors::ErrorResponseBody)
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_book_catalog(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
) -> Result<Json<Vec<BookResponseBody>>, ApiError> {
    let books = deps.catalog.queries.get_book_catalog().await.map_err(service_error)?;
    Ok(Json(books.into_iter().map(BookResponseBody::from).collect()))
}
