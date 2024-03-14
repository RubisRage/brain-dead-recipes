use axum::{routing::get, Router};
use sqlx::SqlitePool;
use tower_http::{services::ServeDir, trace::TraceLayer};

mod models;
mod pages;
mod templates;

#[tokio::main]
async fn main() {
    let db = sqlx::SqlitePool::connect("sqlite:./database.db")
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&db).await.unwrap();

    let app = Router::<SqlitePool>::new()
        .route("/", get(pages::index))
        .merge(pages::recipe::routes())
        .with_state(db)
        .nest_service("/assets", ServeDir::new("dist"))
        .nest_service("/images", ServeDir::new("images"))
        .layer(TraceLayer::new_for_http());

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
