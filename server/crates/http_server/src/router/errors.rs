use anyhow::Error;
use axum::{http::StatusCode, Json};
use domain::{book::BookError, book_copy::BookCopyError, loan::LoanError, member::MemberError};
use serde::Serialize;
use server_bootstrap::CommandError;
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
    tracing::error!("Unhandled request error: {error:?}");
    internal_server_error()
}

#[must_use]
fn conflict(message: impl Into<String>) -> ApiError {
    (
        StatusCode::CONFLICT,
        Json(ErrorResponseBody {
            error: message.into(),
        }),
    )
}

#[must_use]
fn internal_server_error() -> ApiError {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponseBody {
            error: String::from("Something went wrong"),
        }),
    )
}

#[must_use]
pub fn command_error(error: CommandError) -> ApiError {
    match error {
        CommandError::Member(e) => member_error(e),
        CommandError::BookCopy(e) => book_copy_error(e),
        CommandError::Book(e) => book_error(e),
        CommandError::Loan(e) => loan_error(e),
        CommandError::Unexpected(e) => {
            tracing::error!("Unhandled request error: {e:?}");
            internal_server_error()
        }
    }
}

fn member_error(e: MemberError) -> ApiError {
    match e {
        MemberError::NotFound => not_found("Member not found"),
        MemberError::CannotBeSuspended
        | MemberError::CannotBeReactivated
        | MemberError::CannotBorrowWhileSuspended
        | MemberError::LoanLimitReached => conflict(e.to_string()),
    }
}

fn book_copy_error(e: BookCopyError) -> ApiError {
    match e {
        BookCopyError::NotFound => not_found("Book copy not found"),
        BookCopyError::CannotBeBorrowed
        | BookCopyError::CannotBeSentToMaintenance
        | BookCopyError::CannotBeReturnedFromMaintenance
        | BookCopyError::CannotMarkBookLost
        | BookCopyError::CannotBeReturnedFromLost => conflict(e.to_string()),
    }
}

fn book_error(e: BookError) -> ApiError {
    match e {
        BookError::NotFound => not_found("Book not found"),
    }
}

fn loan_error(e: LoanError) -> ApiError {
    match e {
        LoanError::NoActiveLoanForBookCopy | LoanError::CannotBeReturned => conflict(e.to_string()),
    }
}
