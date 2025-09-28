//! API utilities and middleware

use axum::{
    http::{HeaderValue, Method},
    Router,
};
use tower_http::cors::CorsLayer;

/// Configure CORS for the API
#[allow(dead_code)]
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ])
}

/// Create the main API router
#[allow(dead_code)]
pub fn create_router() -> Router {
    Router::new()
        .route("/", axum::routing::get(crate::handlers::root))
        .route("/api/health", axum::routing::get(crate::handlers::health))
        .route(
            "/api/compare",
            axum::routing::post(crate::handlers::compare),
        )
        .layer(cors_layer())
}
