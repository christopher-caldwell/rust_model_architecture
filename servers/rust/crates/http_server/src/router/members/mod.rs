pub mod delete_handlers;
pub mod get_handlers;
pub mod post_handlers;
pub mod put_handlers;
pub mod schemas;

pub use delete_handlers::reactivate_member;
pub use get_handlers::{get_member_details, get_member_loans};
pub use post_handlers::register_member;
pub use put_handlers::suspend_member;
pub use schemas::{
    CreateMemberRequestBody, MemberResponseBody, MEMBERS_PATH, MEMBERS_TAG, MEMBER_BY_ID_PATH,
    MEMBER_LOANS_PATH, MEMBER_SUSPENSION_PATH,
};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_handlers::get_member_details,
        get_handlers::get_member_loans,
        post_handlers::register_member,
        put_handlers::suspend_member,
        delete_handlers::reactivate_member
    ),
    components(schemas(
        CreateMemberRequestBody,
        MemberResponseBody,
        crate::router::loan::LoanResponseBody,
        crate::router::errors::ErrorResponseBody
    ))
)]
pub struct MembershipApi;
