mod extract;
mod handlers;

pub use handlers::routes;

use askama_axum::IntoResponse;
use axum::http::StatusCode;

pub struct AppError(anyhow::Error);

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self {
        Self(e.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> askama_axum::Response {
        let msg = self.0.root_cause().to_string();
        let backtrace = self.0.backtrace().to_string();
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("{}: {}", msg, backtrace),
        )
            .into_response()
    }
}
