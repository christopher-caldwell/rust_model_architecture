pub(crate) mod read_repo;
pub(crate) mod write_repo;

use anyhow::{bail, Result};
use domain::book_copy::BookCopyStatus;

pub use read_repo::BookCopyReadRepoSql;

pub(crate) fn parse_book_copy_status(status: &str) -> Result<BookCopyStatus> {
    match status {
        "active" => Ok(BookCopyStatus::Active),
        "maintenance" => Ok(BookCopyStatus::Maintenance),
        "lost" => Ok(BookCopyStatus::Lost),
        _ => bail!("Unknown book copy status '{status}'"),
    }
}

pub(crate) fn book_copy_status_ident(status: &BookCopyStatus) -> &'static str {
    match status {
        BookCopyStatus::Active => "active",
        BookCopyStatus::Maintenance => "maintenance",
        BookCopyStatus::Lost => "lost",
    }
}
