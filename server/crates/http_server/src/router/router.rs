use crate::router::auth::auth_middleware;
use crate::router::catalog::BOOKS_PATH;
use crate::router::catalog::{
    add_book, add_book_copy, complete_book_copy_maintenance, get_book_catalog,
    get_book_copy_details, mark_book_copy_found, mark_book_copy_lost, report_lost_loaned_book_copy,
    return_book_copy, send_book_copy_to_maintenance, BOOK_COPIES_PATH, BOOK_COPY_BY_ID_PATH,
    BOOK_COPY_LOSS_PATH, BOOK_COPY_LOSS_REPORTS_PATH, BOOK_COPY_MAINTENANCE_PATH,
    BOOK_COPY_RETURNS_PATH,
};
use crate::router::cors::get_cors;
use crate::router::lending::{
    check_out_book_copy, get_overdue_loans, LOANS_PATH, OVERDUE_LOANS_PATH,
};
use crate::router::membership::{
    get_member_details, get_member_loans, reactivate_member, register_member, suspend_member,
    MEMBERS_PATH, MEMBER_BY_ID_PATH, MEMBER_LOANS_PATH, MEMBER_SUSPENSION_PATH,
};
use axum::middleware::from_fn_with_state;
use axum::{
    routing::{get, post, put},
    Router,
};
use server_bootstrap::ServerDeps;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = crate::router::catalog::books::BOOKS_PATH, api = crate::router::catalog::books::BooksApi),
        (path = crate::router::catalog::book_copies::BOOK_COPIES_PATH, api = crate::router::catalog::book_copies::BookCopiesApi),
        (path = crate::router::membership::MEMBERS_PATH, api = crate::router::membership::MembershipApi),
        (path = crate::router::lending::LOANS_PATH, api = crate::router::lending::LendingApi),
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
        .route(BOOKS_PATH, get(get_book_catalog).post(add_book))
        .route(BOOK_COPIES_PATH, post(add_book_copy))
        .route(BOOK_COPY_BY_ID_PATH, get(get_book_copy_details))
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
            post(report_lost_loaned_book_copy),
        )
        .route(MEMBERS_PATH, post(register_member))
        .route(MEMBER_BY_ID_PATH, get(get_member_details))
        .route(
            MEMBER_SUSPENSION_PATH,
            put(suspend_member).delete(reactivate_member),
        )
        .route(MEMBER_LOANS_PATH, get(get_member_loans))
        .route(LOANS_PATH, post(check_out_book_copy))
        .route(OVERDUE_LOANS_PATH, get(get_overdue_loans))
        .layer(from_fn_with_state(deps.clone(), auth_middleware));

    Router::new()
        .merge(swagger_router)
        .merge(public_router)
        .merge(protected_router)
        .layer(cors_layer)
        .with_state(deps)
}
