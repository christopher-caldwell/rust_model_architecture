use application::{ContactInquiryCommands, ContactInquiryQueries};
use std::sync::Arc;

#[derive(Clone)]
pub struct ServerDeps {
    pub auth: AuthDeps,
    pub contact_inquiry: ContactInquiryDeps,
}

#[derive(Clone)]
pub struct AuthDeps {
    pub jwt_secret: String,
}

#[derive(Clone)]
pub struct ContactInquiryDeps {
    pub commands: Arc<ContactInquiryCommands>,
    pub queries: Arc<ContactInquiryQueries>,
}
