use axum::extract::Request;
use axum::response::{Html, IntoResponse};
use tracing::{Level, event};

pub async fn interceptor_handler(request: Request) -> impl IntoResponse {
    event!(Level::INFO, "Request received {:?}", request);
    Html("OK".to_string())
}
