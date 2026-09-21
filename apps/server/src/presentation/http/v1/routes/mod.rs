use axum::Router;
use tower_http::trace::TraceLayer;

use crate::{
    application::state::AppState,
    presentation::http::v1::{
        middlewares::cors::cors,
        routes::{docs::openapi_router, health::health_router},
    },
};

pub mod docs;
pub mod health;

pub fn create_routers(app_state: AppState) -> Router<()> {
    let docs_enabled = app_state.mode == "dev";

    let api_routes = Router::new().merge(health_router());

    let app = Router::new()
        .nest("/api/v1", api_routes)
        .layer(cors(&app_state.web_base_url))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    if docs_enabled {
        app.merge(openapi_router())
    } else {
        app
    }
}
