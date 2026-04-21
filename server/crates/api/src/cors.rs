use http::{HeaderName, Method};
use tower_http::cors::CorsLayer;

/// Gets the CORS configuration for the server.
///
/// # Panics
///
/// Panics if the hardcoded allowed origins are not valid URLs.
pub fn get_cors() -> CorsLayer {
    if cfg!(debug_assertions) {
        CorsLayer::very_permissive()
    } else {
        let allowed_origins = Vec::from([
            "https://craftcode.solutions".parse().unwrap(),
            "https://ops.craftcode.solutions".parse().unwrap(),
            "https://connect.craftcode.solutions".parse().unwrap(),
        ]);
        CorsLayer::new()
            .allow_origin(allowed_origins)
            .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
            .allow_headers([
                HeaderName::from_static("content-type"),
                HeaderName::from_static("authorization"),
            ])
    }
}
