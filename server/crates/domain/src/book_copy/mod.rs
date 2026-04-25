mod entity;
mod enums;
mod errors;
mod logic;
pub mod port;

pub use entity::{BookCopy, BookCopyCreationPayload, BookCopyId, BookCopyPrepared};
pub use enums::{BookCopyStatus, ParseBookCopyStatusError};
pub use errors::BookCopyError;
