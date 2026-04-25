use axum::{extract::State, http::StatusCode, Json};
use server_bootstrap::{CheckOutBookCopyInput, ServerDeps};

use crate::router::{
    auth::AuthUser,
    errors::{command_error, ApiError},
    loan::schemas::{CreateLoanRequestBody, LoanResponseBody, LOANS_TAG},
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
pub async fn check_out_book_copy(
    AuthUser(_claims): AuthUser,
    State(deps): State<ServerDeps>,
    Json(body): Json<CreateLoanRequestBody>,
) -> Result<(StatusCode, Json<LoanResponseBody>), ApiError> {
    let input = CheckOutBookCopyInput {
        member_ident: body.member_ident,
        book_copy_barcode: body.book_copy_barcode,
    };
    let loan = deps.lending.commands.check_out_book_copy(input).await.map_err(command_error)?;
    Ok((StatusCode::CREATED, Json(LoanResponseBody::from(loan))))
}
