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
    delete,
    path = "/{barcode}/loss",
    tag = BOOK_COPIES_TAG,
    params(
        ("barcode" = String, Path, description = "Barcode identifier for the book copy")
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
    Path(barcode): Path<String>,
) -> Result<Json<BookCopyResponseBody>, ApiError> {
    let updated = deps.catalog.commands.mark_book_copy_found(barcode).await.map_err(command_error)?;
    Ok(Json(BookCopyResponseBody::from(updated)))
}

#[utoipa::path(
    delete,
    path = "/{barcode}/maintenance",
    tag = BOOK_COPIES_TAG,
    params(
        ("barcode" = String, Path, description = "Barcode identifier for the book copy")
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
    Path(barcode): Path<String>,
) -> Result<Json<BookCopyResponseBody>, ApiError> {
    let updated = deps
        .catalog
        .commands
        .complete_book_copy_maintenance(barcode)
        .await
        .map_err(command_error)?;
    Ok(Json(BookCopyResponseBody::from(updated)))
}
