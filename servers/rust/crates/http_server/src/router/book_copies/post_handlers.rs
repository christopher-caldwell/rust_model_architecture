use axum::{
    extract::{Path, State},
    Json,
};
use server_bootstrap::ServerDeps;

use crate::router::{
    auth::AuthUser,
    book_copies::schemas::{BookCopyResponseBody, BOOK_COPIES_TAG},
    errors::{command_error, ApiError},
    loan::schemas::LoanResponseBody,
};

#[utoipa::path(
    post,
    path = "/{barcode}/return",
    tag = BOOK_COPIES_TAG,
    params(
        ("barcode" = String, Path, description = "Barcode identifier for the book copy")
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
    Path(barcode): Path<String>,
) -> Result<Json<LoanResponseBody>, ApiError> {
    let return_book_copy_result = deps.lending.commands.return_book_copy(barcode).await;

    let loan_response = match return_book_copy_result {
        Ok(loan) => Json(LoanResponseBody::from(loan)),
        Err(error) => return Err(command_error(error)),
    };

    Ok(loan_response)
}

#[utoipa::path(
    post,
    path = "/{barcode}/report-loss",
    tag = BOOK_COPIES_TAG,
    params(
        ("barcode" = String, Path, description = "Barcode identifier for the book copy")
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
pub async fn report_lost_loaned_book_copy(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Path(barcode): Path<String>,
) -> Result<Json<BookCopyResponseBody>, ApiError> {
    let report_lost_loaned_book_copy_result = deps
        .lending
        .commands
        .report_lost_loaned_book_copy(barcode)
        .await;

    let book_copy_response = match report_lost_loaned_book_copy_result {
        Ok(updated) => Json(BookCopyResponseBody::from(updated)),
        Err(error) => return Err(command_error(error)),
    };

    Ok(book_copy_response)
}
