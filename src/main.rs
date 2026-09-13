use axum::{Router, routing::get};
use tracing::{Level, event};

mod handlers;
use handlers::health_handler;
use handlers::interceptor_handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let app = Router::new()
        .route("/", get(interceptor_handler))
        .route(
            "/weatherstation/updateweatherstation.php",
            get(interceptor_handler),
        )
        .route("/health", get(health_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    event!(Level::INFO, "Server started on 0.0.0.0:8080");

    axum::serve(listener, app).await.unwrap();
}
