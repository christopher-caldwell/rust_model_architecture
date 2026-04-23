use crate::router::auth::auth_middleware;
use crate::router::cors::get_cors;
use crate::router::dependencies::ServerDeps;
use crate::router::graphql::{build_schema, graphql_handler, graphql_playground, GRAPHQL_PATH};
use axum::middleware::from_fn_with_state;
use axum::routing::post;
use axum::{routing::get, Router};

pub fn new_router(deps: ServerDeps) -> Router {
    let cors_layer = get_cors();
    let schema = build_schema(deps.clone());

    let public_router = Router::new().route(GRAPHQL_PATH, get(graphql_playground));

    let protected_router = Router::new()
        .route(GRAPHQL_PATH, post(graphql_handler))
        .with_state(schema)
        .layer(from_fn_with_state(deps.clone(), auth_middleware));

    Router::new()
        .merge(public_router)
        .merge(protected_router)
        .layer(cors_layer)
}
