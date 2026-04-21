#[allow(clippy::module_inception)]
mod queries;
mod port;

pub use port::{ContactInquiryReadRepoPort, ContactInquiryStatusRepoPort};
pub use queries::ContactInquiryQueries;
