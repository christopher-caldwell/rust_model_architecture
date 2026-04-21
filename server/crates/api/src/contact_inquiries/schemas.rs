use chrono::{DateTime, Utc};
use domain::contact_inquiry::{ContactInquiry, ContactInquiryCreationPayload};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// --------------------------------------------------------------------------------------------//
// --------------------------------------------------------------------------------------------//

pub const CONTACT_INQUIRIES_TAG: &str = "Contact Inquiries";
pub const CONTACT_INQUIRIES_PATH: &str = "/contact-inquiries";
pub const CONTACT_INQUIRY_BY_IDENT_PATH: &str = "/contact-inquiries/{ident}";

// --------------------------------------------------------------------------------------------//
// --------------------------------------------------------------------------------------------//
#[derive(Serialize, ToSchema)]
pub struct ContactInquiryResponseBody {
    pub ident: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub status: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub source: String,
    pub website_given: String,
    pub message: String,
}

impl From<ContactInquiry> for ContactInquiryResponseBody {
    fn from(value: ContactInquiry) -> Self {
        Self {
            ident: value.ident,
            dt_created: value.dt_created,
            dt_modified: value.dt_modified,
            status: value.status,
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            phone_number: value.phone_number,
            source: value.source,
            website_given: value.website_given,
            message: value.message,
        }
    }
}

// --------------------------------------------------------------------------------------------//
// --------------------------------------------------------------------------------------------//

#[derive(Deserialize, ToSchema)]
pub struct CreateContactInquiryRequestBody {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub website_given: String,
    pub message: String,
    pub phone_number: Option<String>,
    pub source: String,
    /// honey pot field
    #[schema(ignore)]
    pub url: Option<String>,
}

impl From<CreateContactInquiryRequestBody> for ContactInquiryCreationPayload {
    fn from(value: CreateContactInquiryRequestBody) -> Self {
        let phone_number = value.phone_number.unwrap_or_default();
        Self {
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            phone_number,
            source: value.source,
            website_given: value.website_given,
            message: value.message,
        }
    }
}

// --------------------------------------------------------------------------------------------//
// --------------------------------------------------------------------------------------------//
