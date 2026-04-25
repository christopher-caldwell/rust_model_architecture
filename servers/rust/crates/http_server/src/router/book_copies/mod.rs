pub mod delete_handlers;
pub mod get_handlers;
pub mod post_handlers;
pub mod put_handlers;
pub mod schemas;

pub use delete_handlers::{complete_book_copy_maintenance, mark_book_copy_found};
pub use get_handlers::get_book_copy_details;
pub use post_handlers::{report_lost_loaned_book_copy, return_book_copy};
pub use put_handlers::{mark_book_copy_lost, send_book_copy_to_maintenance};
pub use schemas::{
    BookCopyResponseBody, CreateBookCopyRequestBody, BOOK_COPIES_PATH, BOOK_COPIES_TAG,
    BOOK_COPY_BY_ID_PATH, BOOK_COPY_LOST_PATH, BOOK_COPY_MAINTENANCE_PATH,
    BOOK_COPY_REPORT_LOSS_PATH, BOOK_COPY_RETURN_PATH,
};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_handlers::get_book_copy_details,
        put_handlers::mark_book_copy_lost,
        delete_handlers::mark_book_copy_found,
        put_handlers::send_book_copy_to_maintenance,
        delete_handlers::complete_book_copy_maintenance,
        post_handlers::return_book_copy,
        post_handlers::report_lost_loaned_book_copy
    ),
    components(schemas(
        BookCopyResponseBody,
        CreateBookCopyRequestBody,
        crate::router::errors::ErrorResponseBody
    ))
)]
pub struct BookCopiesApi;
