pub mod get_handlers;
pub mod post_handlers;
pub mod schemas;

pub use get_handlers::get_books;
pub use post_handlers::create_book;
pub use schemas::{BookResponseBody, CreateBookRequestBody, BOOKS_PATH, BOOKS_TAG};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_handlers::get_books, post_handlers::create_book),
    components(schemas(
        BookResponseBody,
        CreateBookRequestBody,
        crate::router::errors::ErrorResponseBody
    ))
)]
pub struct BooksApi;
