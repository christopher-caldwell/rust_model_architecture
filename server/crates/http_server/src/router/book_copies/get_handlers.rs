use axum::{
    extract::{Path, State},
    Json,
};
use domain::book_copy::BookCopyId;

use crate::router::{
    auth::AuthUser,
    book_copies::schemas::{BookCopyResponseBody, BOOK_COPIES_TAG},
    dependencies::ServerDeps,
    errors::{not_found, service_error, ApiError},
};

#[utoipa::path(
    get,
    path = "/{id}",
    tag = BOOK_COPIES_TAG,
    params(
        ("id" = i64, Path, description = "Database identifier for the book copy")
    ),
    responses(
        (status = 200, description = "Book copy details", body = BookCopyResponseBody),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Book copy not found", body = crate::router::errors::ErrorResponseBody),
        (status = 500, description = "Internal server error", body = crate::router::errors::ErrorResponseBody)
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_book_copy_by_id(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Path(id): Path<i64>,
) -> Result<Json<BookCopyResponseBody>, ApiError> {
    let book_copy_result = deps
        .catalog
        .queries
        .get_book_copy_details(BookCopyId(id))
        .await;

    let book_copy_response = match book_copy_result {
        Ok(Some(book_copy)) => BookCopyResponseBody::from(book_copy),
        Ok(None) => return Err(not_found("Book copy not found")),
        Err(error) => return Err(service_error(error)),
    };

    Ok(Json(book_copy_response))
}
