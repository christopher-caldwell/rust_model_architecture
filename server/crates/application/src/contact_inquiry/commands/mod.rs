#[allow(clippy::module_inception)]
mod commands;
mod port;

pub use commands::ContactInquiryCommands;
pub use port::{ContactInquirySpamRatingPort, ContactInquiryWriteRepoPort};
