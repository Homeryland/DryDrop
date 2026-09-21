use axum::response::IntoResponse;

use crate::presentation::http::v1::response::{ApiResponse, Empty};

#[utoipa::path(
    get,
    path = "/api/v1/health",
    tag = "health",
    responses(
        (
            status = 200,
            description = "Server is healthy",
            body = ApiResponse<Empty>
        )
    )
)]
pub async fn health_handler() -> impl IntoResponse {
    let msg = String::from("healthy");
    ApiResponse::<Empty>::success(Some(msg), None)
}
