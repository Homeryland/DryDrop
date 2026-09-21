use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<D> {
    pub code: u16,
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<D>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Empty;

impl<D> ApiResponse<D> {
    pub fn success(message: Option<String>, data: Option<D>) -> Self {
        Self {
            code: 200,
            message,
            data,
        }
    }

    pub fn error(code: u16, message: Option<String>) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }
}

impl<D> IntoResponse for ApiResponse<D>
where
    D: Serialize,
{
    fn into_response(self) -> Response {
        let status = if self.code == 200 {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        };

        (status, Json(self)).into_response()
    }
}
