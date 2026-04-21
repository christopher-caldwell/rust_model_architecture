mod commands;
mod queries;

pub use commands::{
    ContactInquiryCommands, ContactInquirySpamRatingPort, ContactInquiryWriteRepoPort,
};
pub use queries::{
    ContactInquiryQueries, ContactInquiryReadRepoPort, ContactInquiryStatusRepoPort,
};
