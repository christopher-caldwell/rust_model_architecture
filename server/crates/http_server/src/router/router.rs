use crate::router::auth::auth_middleware;
use crate::router::book_copies::{
    complete_book_copy_maintenance, get_book_copy_by_id, mark_book_copy_found, mark_book_copy_lost,
    report_book_copy_lost_on_loan, return_book_copy, send_book_copy_to_maintenance,
    BOOK_COPY_BY_ID_PATH, BOOK_COPY_LOSS_PATH, BOOK_COPY_LOSS_REPORTS_PATH,
    BOOK_COPY_MAINTENANCE_PATH, BOOK_COPY_RETURNS_PATH,
};
use crate::router::books::{
    create_book, create_book_copy, get_books, BOOKS_PATH, BOOK_COPIES_BY_BOOK_ID_PATH,
};
use crate::router::cors::get_cors;
use crate::router::dependencies::ServerDeps;
use crate::router::loans::{create_loan, get_overdue_loans, LOANS_PATH, OVERDUE_LOANS_PATH};
use crate::router::members::{
    create_member, get_member_by_id, get_member_loans, reactivate_member, suspend_member,
    MEMBERS_PATH, MEMBER_BY_ID_PATH, MEMBER_LOANS_PATH, MEMBER_SUSPENSION_PATH,
};
use axum::middleware::from_fn_with_state;
use axum::{
    routing::{get, post, put},
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = crate::router::books::BOOKS_PATH, api = crate::router::books::BooksApi),
        (path = crate::router::book_copies::BOOK_COPIES_PATH, api = crate::router::book_copies::BookCopiesApi),
        (path = crate::router::members::MEMBERS_PATH, api = crate::router::members::MembersApi),
        (path = crate::router::loans::LOANS_PATH, api = crate::router::loans::LoansApi),
        (path = crate::router::health::HEALTH_CHECK_PATH, api = crate::router::health::HealthCheckApi)
    ),
    info(
        title = "Demo Library",
        version = "1.0.0",
        description = "Demo of a library system",
        contact(name = "", email = "christopher@craftcode.solutions")
    )
)]
pub struct ApiDoc;

pub fn new_router(deps: ServerDeps) -> Router {
    let api = ApiDoc::openapi();

    let cors_layer = get_cors();

    let swagger_router = SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api);
    let public_router = Router::new().route(
        crate::router::health::HEALTH_CHECK_PATH,
        get(crate::router::health::get_health_check),
    );

    let protected_router = Router::new()
        .route(BOOKS_PATH, get(get_books).post(create_book))
        .route(BOOK_COPIES_BY_BOOK_ID_PATH, post(create_book_copy))
        .route(BOOK_COPY_BY_ID_PATH, get(get_book_copy_by_id))
        .route(
            BOOK_COPY_LOSS_PATH,
            put(mark_book_copy_lost).delete(mark_book_copy_found),
        )
        .route(
            BOOK_COPY_MAINTENANCE_PATH,
            put(send_book_copy_to_maintenance).delete(complete_book_copy_maintenance),
        )
        .route(BOOK_COPY_RETURNS_PATH, post(return_book_copy))
        .route(
            BOOK_COPY_LOSS_REPORTS_PATH,
            post(report_book_copy_lost_on_loan),
        )
        .route(MEMBERS_PATH, post(create_member))
        .route(MEMBER_BY_ID_PATH, get(get_member_by_id))
        .route(
            MEMBER_SUSPENSION_PATH,
            put(suspend_member).delete(reactivate_member),
        )
        .route(MEMBER_LOANS_PATH, get(get_member_loans))
        .route(LOANS_PATH, post(create_loan))
        .route(OVERDUE_LOANS_PATH, get(get_overdue_loans))
        .layer(from_fn_with_state(deps.clone(), auth_middleware));

    Router::new()
        .merge(swagger_router)
        .merge(public_router)
        .merge(protected_router)
        .layer(cors_layer)
        .with_state(deps)
}
