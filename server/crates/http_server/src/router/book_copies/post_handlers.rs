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
    loans::schemas::LoanResponseBody,
};

#[utoipa::path(
    post,
    path = "/{id}/returns",
    tag = BOOK_COPIES_TAG,
    params(
        ("id" = i64, Path, description = "Identifier for the book copy")
    ),
    responses(
        (status = 200, description = "Book copy returned", body = LoanResponseBody),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Book copy not found", body = crate::router::errors::ErrorResponseBody),
        (status = 409, description = "Book copy cannot be returned", body = crate::router::errors::ErrorResponseBody),
        (status = 500, description = "Internal server error", body = crate::router::errors::ErrorResponseBody)
    ),
    security(("bearer_auth" = []))
)]
pub async fn return_book_copy(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Path(id): Path<i64>,
) -> Result<Json<LoanResponseBody>, ApiError> {
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

    deps.lending
        .commands
        .return_book_copy(book_copy)
        .await
        .map(|loan| Json(LoanResponseBody::from(loan)))
        .map_err(service_error)
}

#[utoipa::path(
    post,
    path = "/{id}/loss-reports",
    tag = BOOK_COPIES_TAG,
    params(
        ("id" = i64, Path, description = "Identifier for the book copy")
    ),
    responses(
        (status = 200, description = "Loaned book copy reported lost", body = BookCopyResponseBody),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Book copy not found", body = crate::router::errors::ErrorResponseBody),
        (status = 409, description = "Book copy cannot be reported lost", body = crate::router::errors::ErrorResponseBody),
        (status = 500, description = "Internal server error", body = crate::router::errors::ErrorResponseBody)
    ),
    security(("bearer_auth" = []))
)]
pub async fn report_book_copy_lost_on_loan(
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

    deps.lending
        .commands
        .report_lost_loaned_book_copy(book_copy)
        .await
        .map(|updated| Json(BookCopyResponseBody::from(updated)))
        .map_err(service_error)
}
