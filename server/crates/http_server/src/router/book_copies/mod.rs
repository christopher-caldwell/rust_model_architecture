pub mod delete_handlers;
pub mod get_handlers;
pub mod post_handlers;
pub mod put_handlers;
pub mod schemas;

pub use delete_handlers::{complete_book_copy_maintenance, mark_book_copy_found};
pub use get_handlers::get_book_copy_by_id;
pub use post_handlers::{report_book_copy_lost_on_loan, return_book_copy};
pub use put_handlers::{mark_book_copy_lost, send_book_copy_to_maintenance};
pub use schemas::{
    BookCopyResponseBody, BOOK_COPIES_PATH, BOOK_COPIES_TAG, BOOK_COPY_BY_ID_PATH,
    BOOK_COPY_LOSS_PATH, BOOK_COPY_LOSS_REPORTS_PATH, BOOK_COPY_MAINTENANCE_PATH,
    BOOK_COPY_RETURNS_PATH,
};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_handlers::get_book_copy_by_id,
        put_handlers::mark_book_copy_lost,
        delete_handlers::mark_book_copy_found,
        put_handlers::send_book_copy_to_maintenance,
        delete_handlers::complete_book_copy_maintenance,
        post_handlers::return_book_copy,
        post_handlers::report_book_copy_lost_on_loan
    ),
    components(schemas(BookCopyResponseBody, crate::router::errors::ErrorResponseBody))
)]
pub struct BookCopiesApi;
