use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use sea_orm::DbErr;
use serde_json::{json, Value};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Something went wrong!")]
    DbErr(#[from] DbErr),
}

pub type Result<T> = std::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::DbErr(_) => internal_error(),
        }
    }
}

fn internal_error() -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        error(
            "Oops! Something went wrong. Please try again after some time...",
        ),
    )
        .into_response()
}

fn error(msg: &'static str) -> Json<Value> {
    Json(json!({ "error": msg }))
}
