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
    delete,
    path = "/{id}/loss",
    tag = BOOK_COPIES_TAG,
    params(
        ("id" = i64, Path, description = "Identifier for the book copy")
    ),
    responses(
        (status = 200, description = "Book copy restored from lost", body = BookCopyResponseBody),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Book copy not found", body = crate::router::errors::ErrorResponseBody),
        (status = 409, description = "Book copy cannot be restored from lost", body = crate::router::errors::ErrorResponseBody),
        (status = 500, description = "Internal server error", body = crate::router::errors::ErrorResponseBody)
    ),
    security(("bearer_auth" = []))
)]
pub async fn mark_book_copy_found(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Path(id): Path<i64>,
) -> Result<Json<BookCopyResponseBody>, ApiError> {
    let book_copy = match deps
        .catalog
        .queries
        .get_book_copy_details(BookCopyId(id))
        .await
    {
        Ok(Some(book_copy)) => book_copy,
        Ok(None) => return Err(not_found("Book copy not found")),
        Err(error) => return Err(service_error(error)),
    };

    deps.catalog
        .commands
        .mark_book_copy_found(book_copy)
        .await
        .map(|updated| Json(BookCopyResponseBody::from(updated)))
        .map_err(service_error)
}

#[utoipa::path(
    delete,
    path = "/{id}/maintenance",
    tag = BOOK_COPIES_TAG,
    params(
        ("id" = i64, Path, description = "Identifier for the book copy")
    ),
    responses(
        (status = 200, description = "Book copy returned from maintenance", body = BookCopyResponseBody),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Book copy not found", body = crate::router::errors::ErrorResponseBody),
        (status = 409, description = "Book copy cannot be returned from maintenance", body = crate::router::errors::ErrorResponseBody),
        (status = 500, description = "Internal server error", body = crate::router::errors::ErrorResponseBody)
    ),
    security(("bearer_auth" = []))
)]
pub async fn complete_book_copy_maintenance(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Path(id): Path<i64>,
) -> Result<Json<BookCopyResponseBody>, ApiError> {
    let book_copy = match deps
        .catalog
        .queries
        .get_book_copy_details(BookCopyId(id))
        .await
    {
        Ok(Some(book_copy)) => book_copy,
        Ok(None) => return Err(not_found("Book copy not found")),
        Err(error) => return Err(service_error(error)),
    };

    deps.catalog
        .commands
        .complete_book_copy_maintenance(book_copy)
        .await
        .map(|updated| Json(BookCopyResponseBody::from(updated)))
        .map_err(service_error)
}
