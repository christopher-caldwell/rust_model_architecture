use axum::{
    extract::{Path, State},
    Json,
};
use server_bootstrap::ServerDeps;

use crate::router::{
    auth::AuthUser,
    book_copies::schemas::{BookCopyResponseBody, BOOK_COPIES_TAG},
    errors::{command_error, ApiError},
};

#[utoipa::path(
    put,
    path = "/{barcode}/lost",
    tag = BOOK_COPIES_TAG,
    params(
        ("barcode" = String, Path, description = "Barcode identifier for the book copy")
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
    Path(barcode): Path<String>,
) -> Result<Json<BookCopyResponseBody>, ApiError> {
    let mark_book_copy_lost_result = deps.catalog.commands.mark_book_copy_lost(barcode).await;

    let book_copy_response = match mark_book_copy_lost_result {
        Ok(updated) => Json(BookCopyResponseBody::from(updated)),
        Err(error) => return Err(command_error(error)),
    };

    Ok(book_copy_response)
}

#[utoipa::path(
    put,
    path = "/{barcode}/maintenance",
    tag = BOOK_COPIES_TAG,
    params(
        ("barcode" = String, Path, description = "Barcode identifier for the book copy")
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
    Path(barcode): Path<String>,
) -> Result<Json<BookCopyResponseBody>, ApiError> {
    let send_book_copy_to_maintenance_result = deps
        .catalog
        .commands
        .send_book_copy_to_maintenance(barcode)
        .await;

    let book_copy_response = match send_book_copy_to_maintenance_result {
        Ok(updated) => Json(BookCopyResponseBody::from(updated)),
        Err(error) => return Err(command_error(error)),
    };

    Ok(book_copy_response)
}
