#[derive(thiserror::Error, Debug)]
pub enum BookError {
    #[error("Book not found")]
    NotFound,
}
