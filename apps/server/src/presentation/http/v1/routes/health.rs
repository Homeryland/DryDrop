use axum::{Router, routing::get};

use crate::presentation::http::v1::{handlers::health::health_handler, state::AppState};

pub fn health_router() -> Router<AppState> {
    Router::new().route("/health", get(health_handler))
}
