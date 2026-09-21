use axum::http::{
    self, HeaderValue,
    header::{ACCEPT, AUTHORIZATION},
};
use tower_http::cors::CorsLayer;
use tracing::info;

pub fn cors(web_base_url: &str) -> CorsLayer {
    info!(
        "Cors for {}, available methods: {}, {}, {}, {}, {}, {}, {}, allow headers: {}, {}",
        web_base_url,
        http::Method::GET.as_str(),
        http::Method::POST.as_str(),
        http::Method::PUT.as_str(),
        http::Method::DELETE.as_str(),
        http::Method::PATCH.as_str(),
        http::Method::OPTIONS.as_str(),
        http::Method::HEAD.as_str(),
        AUTHORIZATION.as_str(),
        ACCEPT.as_str(),
    );
    CorsLayer::new()
        .allow_origin(web_base_url.parse::<HeaderValue>().unwrap())
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
            http::Method::PATCH,
            http::Method::OPTIONS,
            http::Method::HEAD,
        ])
        .allow_headers([AUTHORIZATION, ACCEPT])
}
