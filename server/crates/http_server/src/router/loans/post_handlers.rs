use axum::{extract::State, http::StatusCode, Json};
use domain::{book_copy::BookCopyId, member::MemberId};

use crate::router::{
    auth::AuthUser,
    dependencies::ServerDeps,
    errors::{not_found, service_error, ApiError},
    loans::schemas::{CreateLoanRequestBody, LoanResponseBody, LOANS_TAG},
};

#[utoipa::path(
    post,
    path = "",
    tag = LOANS_TAG,
    request_body = CreateLoanRequestBody,
    responses(
        (status = 201, description = "Loan created", body = LoanResponseBody),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Member or book copy not found", body = crate::router::errors::ErrorResponseBody),
        (status = 409, description = "Loan cannot be created", body = crate::router::errors::ErrorResponseBody),
        (status = 500, description = "Internal server error", body = crate::router::errors::ErrorResponseBody)
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_loan(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Json(body): Json<CreateLoanRequestBody>,
) -> Result<(StatusCode, Json<LoanResponseBody>), ApiError> {
    let member_result = deps
        .membership
        .queries
        .get_member_details(MemberId(body.member_id))
        .await;

    let member = match member_result {
        Ok(Some(member)) => member,
        Ok(None) => return Err(not_found("Member not found")),
        Err(error) => return Err(service_error(error)),
    };

    let book_copy_result = deps
        .catalog
        .queries
        .get_book_copy_details(BookCopyId(body.book_copy_id))
        .await;

    let book_copy = match book_copy_result {
        Ok(Some(book_copy)) => book_copy,
        Ok(None) => return Err(not_found("Book copy not found")),
        Err(error) => return Err(service_error(error)),
    };

    let check_out_book_copy_result = deps.lending
        .commands
        .check_out_book_copy(member, book_copy)
        .await;

    let loan_response = match check_out_book_copy_result {
        Ok(loan) => (StatusCode::CREATED, Json(LoanResponseBody::from(loan))),
        Err(error) => return Err(service_error(error)),
    };

    Ok(loan_response)
}
