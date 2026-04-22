use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use domain::book_copy::BookCopyCreationPayload;

use crate::router::{
    auth::AuthUser,
    book_copies::schemas::{BookCopyResponseBody, CreateBookCopyRequestBody, BOOK_COPIES_TAG},
    dependencies::ServerDeps,
    errors::{not_found, service_error, ApiError},
    loans::schemas::LoanResponseBody,
};

#[utoipa::path(
    post,
    path = "/{barcode}/returns",
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
    let book_copy_result = deps.catalog.queries.get_book_copy_details(&barcode).await;

    let book_copy = match book_copy_result {
        Ok(Some(book_copy)) => book_copy,
        Ok(None) => return Err(not_found("Book copy not found")),
        Err(error) => return Err(service_error(error)),
    };

    let return_book_copy_result = deps.lending.commands.return_book_copy(book_copy).await;

    let loan_response = match return_book_copy_result {
        Ok(loan) => Json(LoanResponseBody::from(loan)),
        Err(error) => return Err(service_error(error)),
    };

    Ok(loan_response)
}

#[utoipa::path(
    post,
    path = "/{barcode}/loss-reports",
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
pub async fn report_book_copy_lost_on_loan(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Path(barcode): Path<String>,
) -> Result<Json<BookCopyResponseBody>, ApiError> {
    let book_copy_result = deps.catalog.queries.get_book_copy_details(&barcode).await;

    let book_copy = match book_copy_result {
        Ok(Some(book_copy)) => book_copy,
        Ok(None) => return Err(not_found("Book copy not found")),
        Err(error) => return Err(service_error(error)),
    };

    let report_lost_loaned_book_copy_result = deps
        .lending
        .commands
        .report_lost_loaned_book_copy(book_copy)
        .await;

    let book_copy_response = match report_lost_loaned_book_copy_result {
        Ok(updated) => Json(BookCopyResponseBody::from(updated)),
        Err(error) => return Err(service_error(error)),
    };

    Ok(book_copy_response)
}

#[utoipa::path(
    post,
    path = "",
    tag = BOOK_COPIES_TAG,
    request_body = CreateBookCopyRequestBody,
    responses(
        (status = 201, description = "Book copy created", body = BookCopyResponseBody),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Book not found", body = crate::router::errors::ErrorResponseBody),
        (status = 500, description = "Internal server error", body = crate::router::errors::ErrorResponseBody)
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_book_copy(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Json(body): Json<CreateBookCopyRequestBody>,
) -> Result<(StatusCode, Json<BookCopyResponseBody>), ApiError> {
    let book_result = deps.catalog.queries.get_book_by_isbn(&body.isbn).await;

    let book = match book_result {
        Ok(Some(book)) => book,
        Ok(None) => return Err(not_found("Book not found")),
        Err(error) => return Err(service_error(error)),
    };

    let payload = BookCopyCreationPayload {
        barcode: body.barcode,
        author_name: body.author_name,
        book_id: book.id,
    };

    let add_book_copy_result = deps.catalog.commands.add_book_copy(payload).await;

    let book_copy_response = match add_book_copy_result {
        Ok(book_copy) => Json(BookCopyResponseBody::from(book_copy)),
        Err(error) => return Err(service_error(error)),
    };

    Ok((StatusCode::CREATED, book_copy_response))
}
