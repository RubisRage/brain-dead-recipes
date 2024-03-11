use axum::{routing::get, Router};
use tower_http::{services::ServeDir, trace::TraceLayer};

mod handlers;
mod models;
mod templates;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handlers::index))
        .merge(handlers::recipe::routes())
        .nest_service("/assets", ServeDir::new("dist"))
        .nest_service("/images", ServeDir::new("images"))
        .layer(TraceLayer::new_for_http());

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
