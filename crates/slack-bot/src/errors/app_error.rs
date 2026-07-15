use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Internal Server Error")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, self.to_string()).into_response()
            }
            AppError::Internal => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
        }
    }
}