use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::presentation::http::v1::openapi::ApiDoc;

pub fn openapi_router() -> Router {
    Router::new().merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
