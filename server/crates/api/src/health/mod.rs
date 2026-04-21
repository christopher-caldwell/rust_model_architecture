pub mod get_handlers;
pub mod schemas;

pub use get_handlers::get_health_check;
pub use schemas::{HealthCheckResponseBody, HEALTH_CHECK_PATH, HEALTH_CHECK_TAG};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_handlers::get_health_check,),
    components(schemas(HealthCheckResponseBody))
)]
pub struct HealthCheckApi;
