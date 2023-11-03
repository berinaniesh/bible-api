use thiserror::Error;
use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use sqlx::Error as SqlxError;
use serde_json::json;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("The requested resource was not found in the database")]
    NotFound,

    #[error("Internal Server Error")]
    InternalServerError,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::NotFound => {
                HttpResponse::NotFound().json(json!({
                    "message": self.to_string()
                }))
            }
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().json(json!({
                    "message": self.to_string()
                }))
            }
        }
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::NotFound => {
                StatusCode::NOT_FOUND
            }
            AppError::InternalServerError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        match err {
            SqlxError::RowNotFound | SqlxError::ColumnNotFound(_) => {
                AppError::NotFound
            }
            _ => {
                AppError::InternalServerError
            }
        }
    }
}
