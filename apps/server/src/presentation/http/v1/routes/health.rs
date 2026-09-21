use axum::{Router, routing::get};

use crate::{
    application::state::AppState, presentation::http::v1::handlers::health::health_handler,
};

pub fn health_router() -> Router<AppState> {
    Router::new().route("/health", get(health_handler))
}
