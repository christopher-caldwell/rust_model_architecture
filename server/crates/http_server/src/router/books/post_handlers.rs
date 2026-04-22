use axum::{extract::State, http::StatusCode, Json};

use crate::router::{
    auth::AuthUser,
    books::schemas::{BookResponseBody, CreateBookRequestBody, BOOKS_TAG},
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
    let add_book_result = deps.catalog.commands.add_book(body.into()).await;

    let book_response = match add_book_result {
        Ok(book) => Json(BookResponseBody::from(book)),
        Err(error) => return Err(service_error(error)),
    };

    Ok((StatusCode::CREATED, book_response))
}
