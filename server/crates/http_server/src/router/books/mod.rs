pub mod get_handlers;
pub mod post_handlers;
pub mod schemas;

pub use get_handlers::get_books;
pub use post_handlers::{create_book, create_book_copy};
pub use schemas::{
    BookResponseBody, CreateBookCopyRequestBody, CreateBookRequestBody, BOOKS_PATH, BOOKS_TAG,
    BOOK_COPIES_BY_BOOK_ID_PATH,
};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_handlers::get_books,
        post_handlers::create_book,
        post_handlers::create_book_copy
    ),
    components(schemas(
        BookResponseBody,
        CreateBookRequestBody,
        CreateBookCopyRequestBody,
        crate::router::book_copies::schemas::BookCopyResponseBody,
        crate::router::errors::ErrorResponseBody
    ))
)]
pub struct BooksApi;
