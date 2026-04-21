pub mod get_handlers;
pub mod post_handlers;
pub mod schemas;

pub use get_handlers::{get_contact_inquiries, get_contact_inquiry_by_ident};
pub use post_handlers::create_contact_inquiry;
pub use schemas::{
    ContactInquiryResponseBody, CreateContactInquiryRequestBody, CONTACT_INQUIRY_BY_IDENT_PATH,
    CONTACT_INQUIRIES_PATH, CONTACT_INQUIRIES_TAG,
};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_handlers::get_contact_inquiry_by_ident,
        get_handlers::get_contact_inquiries,
        post_handlers::create_contact_inquiry
    ),
    components(schemas(ContactInquiryResponseBody, CreateContactInquiryRequestBody))
)]
pub struct ContactApi;
