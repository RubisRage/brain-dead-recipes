use axum::{routing::get, Router};
use tower_http::services::ServeDir;

mod handlers;
mod templates;

use templates::{Index, RecipeForm};

async fn index() -> Index {
    Index {}
}

async fn recipe_form() -> RecipeForm {
    RecipeForm {}
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/recipe", get(recipe_form))
        .nest_service("/assets", ServeDir::new("assets"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
