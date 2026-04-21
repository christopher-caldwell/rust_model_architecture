use std::time::Duration;

use crate::{
    contact_inquiries::{schemas::CONTACT_INQUIRIES_TAG, CreateContactInquiryRequestBody},
    dependencies::ServerDeps,
};
use axum::{extract::State, http::StatusCode, Form, Json};
use domain::contact_inquiry::ContactInquiryCreationPayload;
use serde_json::{json, Value};
use tokio::time::sleep;

#[utoipa::path(
    post,
    path = "",
    tag = CONTACT_INQUIRIES_TAG,
    responses(
        (status = 201),
        (status = 422),
        (status = 500)
    )
)]
/// Creates a new contact inquiry.
///
/// # Errors
///
/// Returns `StatusCode::INTERNAL_SERVER_ERROR` if the database operation or spam check fails.
pub async fn create_contact_inquiry(
    State(deps): State<ServerDeps>,
    Form(body): Form<CreateContactInquiryRequestBody>,
) -> Result<(StatusCode, ()), (StatusCode, Json<Value>)> {
    let has_url = body.url.as_ref().is_some_and(|v| !v.is_empty());

    if has_url {
        // Sleep on purpose to simulate creation
        sleep(Duration::from_millis(800)).await;
        tracing::warn!(
            "Honeypot triggered. url: {:?} -- phone_number: {:?}",
            body.url,
            body.phone_number
        );
        return Ok((StatusCode::CREATED, ()));
    }
    let payload = ContactInquiryCreationPayload::from(body);
    let result = deps.contact_inquiry.commands.create(payload).await;
    match result {
        Ok(_) => Ok((StatusCode::CREATED, ())),
        Err(e) => {
            tracing::error!("Failed to create contact inquiry: {e:?}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Something went wrong"})),
            ))
        }
    }
}
