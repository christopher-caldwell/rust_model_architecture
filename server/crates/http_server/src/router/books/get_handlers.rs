use axum::{extract::State, Json};

use crate::router::{
    auth::AuthUser,
    books::schemas::{BookResponseBody, BOOKS_TAG},
    dependencies::ServerDeps,
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
pub async fn get_books(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
) -> Result<Json<Vec<BookResponseBody>>, ApiError> {
    deps.catalog
        .queries
        .get_book_catalog()
        .await
        .map(|books| Json(books.into_iter().map(BookResponseBody::from).collect()))
        .map_err(service_error)
}
