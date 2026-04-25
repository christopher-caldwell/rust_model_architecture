pub mod get_handlers;
pub mod post_handlers;
pub mod schemas;

pub use get_handlers::get_book_catalog;
pub use post_handlers::{add_book, add_book_copy};
pub use schemas::{BookResponseBody, CreateBookRequestBody, BOOKS_PATH, BOOKS_TAG};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_handlers::get_book_catalog,
        post_handlers::add_book,
        post_handlers::add_book_copy
    ),
    components(schemas(
        BookResponseBody,
        CreateBookRequestBody,
        crate::router::errors::ErrorResponseBody
    ))
)]
pub struct BooksApi;
