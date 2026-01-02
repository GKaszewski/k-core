use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tower_sessions::{Expiry, SessionManagerLayer, SessionStore};

pub struct ServerConfig {
    pub cors_origins: Vec<String>,
    pub session_secret: Option<String>,
}

pub fn apply_standard_middleware(app: Router, config: &ServerConfig) -> Router {
    let mut cors = CorsLayer::new()
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PATCH,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
            axum::http::header::CONTENT_TYPE,
        ])
        .allow_credentials(true);

    // Configure CORS origins
    let mut allowed_origins = Vec::new();
    for origin in &config.cors_origins {
        if let Ok(value) = origin.parse::<axum::http::HeaderValue>() {
            allowed_origins.push(value);
        }
    }

    if !allowed_origins.is_empty() {
        cors = cors.allow_origin(allowed_origins);
    }

    app.layer(cors).layer(TraceLayer::new_for_http())
}

/// Helper to attach a session layer with standard K-Suite configuration
pub fn attach_session_layer<S>(app: Router, store: S) -> Router
where
    S: SessionStore + Clone + Send + Sync + 'static,
{
    let session_layer = SessionManagerLayer::new(store)
        .with_secure(false) // Set to true if you handle HTTPS termination in the app
        .with_expiry(Expiry::OnInactivity(time::Duration::days(7)));

    app.layer(session_layer)
}
