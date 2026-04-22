use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use domain::{book::BookId, book_copy::BookCopyCreationPayload};

use crate::router::{
    auth::AuthUser,
    book_copies::schemas::BookCopyResponseBody,
    books::schemas::{
        BookResponseBody, CreateBookCopyRequestBody, CreateBookRequestBody, BOOKS_TAG,
    },
    dependencies::ServerDeps,
    errors::{service_error, ApiError},
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
pub async fn create_book(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Json(body): Json<CreateBookRequestBody>,
) -> Result<(StatusCode, Json<BookResponseBody>), ApiError> {
    deps.catalog
        .commands
        .add_book(body.into())
        .await
        .map(|book| (StatusCode::CREATED, Json(BookResponseBody::from(book))))
        .map_err(service_error)
}

#[utoipa::path(
    post,
    path = "/{book_id}/copies",
    tag = BOOKS_TAG,
    params(
        ("book_id" = i16, Path, description = "Identifier for the book")
    ),
    request_body = CreateBookCopyRequestBody,
    responses(
        (status = 201, description = "Book copy created", body = BookCopyResponseBody),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error", body = crate::router::errors::ErrorResponseBody)
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_book_copy(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Path(book_id): Path<i16>,
    Json(body): Json<CreateBookCopyRequestBody>,
) -> Result<(StatusCode, Json<BookCopyResponseBody>), ApiError> {
    let payload = BookCopyCreationPayload {
        barcode: body.barcode,
        author_name: body.author_name,
        book_id: BookId(book_id),
    };

    deps.catalog
        .commands
        .add_book_copy(payload)
        .await
        .map(|book_copy| {
            (
                StatusCode::CREATED,
                Json(BookCopyResponseBody::from(book_copy)),
            )
        })
        .map_err(service_error)
}
