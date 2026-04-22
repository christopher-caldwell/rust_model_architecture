use anyhow::Error;
use axum::{http::StatusCode, Json};
use domain::{book_copy::BookCopyError, loan::LoanError, member::MemberError};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ErrorResponseBody {
    pub error: String,
}

pub type ApiError = (StatusCode, Json<ErrorResponseBody>);

#[must_use]
pub fn not_found(message: impl Into<String>) -> ApiError {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorResponseBody {
            error: message.into(),
        }),
    )
}

#[must_use]
pub fn service_error(error: Error) -> ApiError {
    if let Some(message) = conflict_message(&error) {
        return (
            StatusCode::CONFLICT,
            Json(ErrorResponseBody { error: message }),
        );
    }

    tracing::error!("Unhandled request error: {error:?}");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponseBody {
            error: String::from("Something went wrong"),
        }),
    )
}

fn conflict_message(error: &Error) -> Option<String> {
    for cause in error.chain() {
        if let Some(inner) = cause.downcast_ref::<BookCopyError>() {
            return Some(inner.to_string());
        }
        if let Some(inner) = cause.downcast_ref::<LoanError>() {
            return Some(inner.to_string());
        }
        if let Some(inner) = cause.downcast_ref::<MemberError>() {
            return Some(inner.to_string());
        }
    }

    None
}
