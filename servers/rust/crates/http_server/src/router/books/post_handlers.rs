use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use server_bootstrap::{AddBookCopyInput, ServerDeps};

use crate::router::{
    auth::AuthUser,
    book_copies::schemas::{BookCopyResponseBody, CreateBookCopyRequestBody},
    books::schemas::{BookResponseBody, CreateBookRequestBody, BOOKS_TAG},
    errors::{command_error, ApiError},
};

#[utoipa::path(
    post,
    path = "",
    tag = BOOKS_TAG,
    request_body = CreateBookRequestBody,
    responses(
        (status = 201, description = "Book created", body = BookResponseBody),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error", body = crate::router::errors::ErrorResponseBody)
    ),
    security(("bearer_auth" = []))
)]
pub async fn add_book(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Json(body): Json<CreateBookRequestBody>,
) -> Result<(StatusCode, Json<BookResponseBody>), ApiError> {
    let book = deps.catalog.commands.add_book(body.into()).await.map_err(command_error)?;
    Ok((StatusCode::CREATED, Json(BookResponseBody::from(book))))
}

#[utoipa::path(
    post,
    path = "/{isbn}/copies",
    tag = BOOKS_TAG,
    params(
        ("isbn" = String, Path, description = "ISBN identifier for the book")
    ),
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
pub async fn add_book_copy(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Path(isbn): Path<String>,
    Json(body): Json<CreateBookCopyRequestBody>,
) -> Result<(StatusCode, Json<BookCopyResponseBody>), ApiError> {
    let input = AddBookCopyInput {
        isbn,
        barcode: body.barcode,
    };
    let book_copy = deps.catalog.commands.add_book_copy(input).await.map_err(command_error)?;
    Ok((StatusCode::CREATED, Json(BookCopyResponseBody::from(book_copy))))
}
