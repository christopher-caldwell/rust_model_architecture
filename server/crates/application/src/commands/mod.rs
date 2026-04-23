pub mod catalog;
pub mod error;
pub mod lending;
pub mod membership;

pub struct AddBookCopyInput {
    pub isbn: String,
    pub barcode: String,
    pub author_name: String,
}

pub struct CheckOutBookCopyInput {
    pub member_ident: String,
    pub book_copy_barcode: String,
}

pub struct MemberIdentInput {
    pub member_ident: String,
}

pub use {
    catalog::CatalogCommands, error::CommandError, lending::LendingCommands,
    membership::MembershipCommands,
};
