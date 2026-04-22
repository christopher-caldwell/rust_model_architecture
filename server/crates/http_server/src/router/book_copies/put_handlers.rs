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
    put,
    path = "/{id}/loss",
    tag = BOOK_COPIES_TAG,
    params(
        ("id" = i64, Path, description = "Identifier for the book copy")
    ),
    responses(
        (status = 200, description = "Book copy marked lost", body = BookCopyResponseBody),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Book copy not found", body = crate::router::errors::ErrorResponseBody),
        (status = 409, description = "Book copy cannot transition to lost", body = crate::router::errors::ErrorResponseBody),
        (status = 500, description = "Internal server error", body = crate::router::errors::ErrorResponseBody)
    ),
    security(("bearer_auth" = []))
)]
pub async fn mark_book_copy_lost(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Path(id): Path<i64>,
) -> Result<Json<BookCopyResponseBody>, ApiError> {
    let book_copy_result = deps
        .catalog
        .queries
        .get_book_copy_details(BookCopyId(id))
        .await;

    let book_copy = match book_copy_result {
        Ok(Some(book_copy)) => book_copy,
        Ok(None) => return Err(not_found("Book copy not found")),
        Err(error) => return Err(service_error(error)),
    };

    let mark_book_copy_lost_result = deps.catalog
        .commands
        .mark_book_copy_lost(book_copy)
        .await;

    let book_copy_response = match mark_book_copy_lost_result {
        Ok(updated) => Json(BookCopyResponseBody::from(updated)),
        Err(error) => return Err(service_error(error)),
    };

    Ok(book_copy_response)
}

#[utoipa::path(
    put,
    path = "/{id}/maintenance",
    tag = BOOK_COPIES_TAG,
    params(
        ("id" = i64, Path, description = "Identifier for the book copy")
    ),
    responses(
        (status = 200, description = "Book copy sent to maintenance", body = BookCopyResponseBody),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Book copy not found", body = crate::router::errors::ErrorResponseBody),
        (status = 409, description = "Book copy cannot enter maintenance", body = crate::router::errors::ErrorResponseBody),
        (status = 500, description = "Internal server error", body = crate::router::errors::ErrorResponseBody)
    ),
    security(("bearer_auth" = []))
)]
pub async fn send_book_copy_to_maintenance(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Path(id): Path<i64>,
) -> Result<Json<BookCopyResponseBody>, ApiError> {
    let book_copy_result = deps
        .catalog
        .queries
        .get_book_copy_details(BookCopyId(id))
        .await;

    let book_copy = match book_copy_result {
        Ok(Some(book_copy)) => book_copy,
        Ok(None) => return Err(not_found("Book copy not found")),
        Err(error) => return Err(service_error(error)),
    };

    let send_book_copy_to_maintenance_result = deps.catalog
        .commands
        .send_book_copy_to_maintenance(book_copy)
        .await;

    let book_copy_response = match send_book_copy_to_maintenance_result {
        Ok(updated) => Json(BookCopyResponseBody::from(updated)),
        Err(error) => return Err(service_error(error)),
    };

    Ok(book_copy_response)
}
