use utoipa::OpenApi;

use crate::presentation::http::v1::handlers::health::__path_health_handler;
use crate::presentation::http::v1::response::{ApiResponse, Empty};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "DryDrop API",
        version = env!("CARGO_PKG_VERSION"),
        description = "DryDrop server HTTP API"
    ),
    paths(health_handler),
    components(schemas(ApiResponse<Empty>, Empty)),
    tags(
        (name = "health", description = "Server health endpoints")
    )
)]
pub struct ApiDoc;
