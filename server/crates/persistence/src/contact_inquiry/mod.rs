pub(crate) mod read_repo;
mod status_repo;
pub(crate) mod write_repo;

pub use read_repo::ContactInquiryReadRepoSql;
pub use status_repo::ContactInquiryStatusRepoSql;
