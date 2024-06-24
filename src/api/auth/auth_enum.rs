use rocket::{http::Status, response::Responder, Request, Response};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MontingerError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Missing credentials")]
    MissingCredentials,

    #[error("Token encoding error: {0}")]
    TokenEncodingError(#[from] jsonwebtoken::errors::Error),

    #[error("Database error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),

    #[error("Client mutex poisoned")]
    PoisonedMutex,

    #[error("User not found")]
    UserNotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Internal Server Error")]
    InternalServerError,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error_type: String,
    message: String,
}

impl<'r> Responder<'r, 'static> for MontingerError {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let (status, error_response) = match self {
            MontingerError::InvalidCredentials | MontingerError::MissingCredentials => (
                Status::Unauthorized, // 401 Unauthorized
                ErrorResponse {
                    error_type: "InvalidCredentials".to_string(),
                    message: "Invalid credentials".to_string(),
                },
            ),
            MontingerError::TokenEncodingError(e) => (
                Status::Unauthorized, // 401 Unauthorized
                ErrorResponse {
                    error_type: "TokenEncodingError".to_string(),
                    message: format!("Token encoding error: {}", e),
                },
            ),
            MontingerError::DatabaseError(e) => (
                Status::InternalServerError, // Use the fully qualified name
                ErrorResponse {
                    error_type: "DatabaseError".to_string(),
                    message: format!("Database error: {}", e),
                },
            ),
            MontingerError::PoisonedMutex => (
                Status::InternalServerError, // Use the fully qualified name
                ErrorResponse {
                    error_type: "PoisonedMutex".to_string(),
                    message: "Client mutex poisoned".to_string(),
                },
            ),
            MontingerError::UserNotFound => (
                Status::NotFound, // 404 Not Found
                ErrorResponse {
                    error_type: "UserNotFound".to_string(),
                    message: "User not found".to_string(),
                },
            ),
            MontingerError::Unauthorized => (
                Status::Unauthorized, // 401 Unauthorized
                ErrorResponse {
                    error_type: "Unauthorized".to_string(),
                    message: "Unauthorized".to_string(),
                },
            ),
            MontingerError::InternalServerError => (
                Status::InternalServerError, // 500 Internal Server Error
                ErrorResponse {
                    error_type: "InternalServerError".to_string(),
                    message: "Internal Server Error".to_string(),
                },
            ),
        };

        Response::build_from(rocket::serde::json::Json(error_response).respond_to(request)?)
            .status(status)
            .ok()
    }
}
