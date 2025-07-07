use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use std::fmt;

/// Define your application error enum
#[derive(Debug)]
pub enum AppError {
    DbError(DieselError),
    NotFound(String),
    Internal(String),
}

/// Allow AppError to be printable (for .to_string())
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DbError(e) => write!(f, "Database error: {}", e),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

/// This allows Actix Web to convert your AppError into HttpResponse
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DbError(_) | AppError::Internal(_) => {
                HttpResponse::InternalServerError().body(self.to_string())
            }
            AppError::NotFound(_) => HttpResponse::NotFound().body(self.to_string()),
        }
    }
}

/// Convert Diesel errors directly to AppError
impl From<DieselError> for AppError {
    fn from(error: DieselError) -> Self {
        if error == DieselError::NotFound {
            AppError::NotFound("Item not found".into())
        } else {
            AppError::DbError(error)
        }
    }
}

