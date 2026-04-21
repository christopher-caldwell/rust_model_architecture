use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    auth::AuthUser,
    contact_inquiries::schemas::{ContactInquiryResponseBody, CONTACT_INQUIRIES_TAG},
    dependencies::ServerDeps,
};

#[utoipa::path(
    get,
    path = "/{ident}",
    tag = CONTACT_INQUIRIES_TAG,
    params(
        ("ident" = String, Path, description = "Unique identifier for the contact inquiry")
    ),
    responses(
        (status = 200, description = "Inquiry found successfully", body = ContactInquiryResponseBody),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Inquiry not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
/// Gets a contact inquiry by its identifier.
///
/// # Errors
///
/// Returns `StatusCode::NOT_FOUND` if the inquiry is not found.
/// Returns `StatusCode::INTERNAL_SERVER_ERROR` if the database operation fails.
pub async fn get_contact_inquiry_by_ident(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Path(ident): Path<String>,
) -> Result<Json<ContactInquiryResponseBody>, StatusCode> {
    let contact_inquiry_result = deps.contact_inquiry.queries.get_by_ident(&ident).await;

    match contact_inquiry_result {
        Ok(Some(contact_inquiry)) => Ok(Json(ContactInquiryResponseBody::from(contact_inquiry))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// ------------------------------------------------------------------------------------------------------------------------------//
// ------------------------------------------------------------------------------------------------------------------------------//

#[utoipa::path(
    get,
    path = "",
    tag = CONTACT_INQUIRIES_TAG,
    responses(
        (status = 200, description = "A list of inquiries", body = Vec<ContactInquiryResponseBody>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
/// Gets all contact inquiries.
///
/// # Errors
///
/// Returns `StatusCode::INTERNAL_SERVER_ERROR` if the database operation fails.
pub async fn get_contact_inquiries(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
) -> Result<Json<Vec<ContactInquiryResponseBody>>, StatusCode> {
    let contact_inquires_result = deps.contact_inquiry.queries.get_contact_inquires().await;

    match contact_inquires_result {
        Ok(contact_inquires) => Ok(Json(
            contact_inquires
                .into_iter()
                .map(ContactInquiryResponseBody::from)
                .collect(),
        )),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// ------------------------------------------------------------------------------------------------------------------------------//
// ------------------------------------------------------------------------------------------------------------------------------//
