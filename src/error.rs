use actix_web::error;
use actix_web::http::StatusCode;
use tera;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum AppError {
   #[error("An internal error occurred. Please try again later.")]
   InternalError(String),
   #[error("The request object was not found.")]
   NotFound,
}

impl error::ResponseError for AppError {
   fn status_code(&self) -> StatusCode {
      match *self {
         AppError::InternalError(_) => {
            StatusCode::INTERNAL_SERVER_ERROR
         }
         AppError::NotFound => StatusCode::NOT_FOUND,
      }
   }
}

impl From<tera::Error> for AppError {
   fn from(error: tera::Error) -> Self {
      log::error!("{}", error.to_string());
      AppError::InternalError(error.to_string())
   }
}

impl From<rusqlite::Error> for AppError {
   fn from(error: rusqlite::Error) -> Self {
      match error {
         rusqlite::Error::QueryReturnedNoRows => AppError::NotFound,
         _ => {
            log::error!("{}", error.to_string());
            AppError::InternalError(error.to_string())
         }
      }
   }
}

impl From<serde_json::Error> for AppError {
   fn from(error: serde_json::Error) -> Self {
      log::error!("{}", error.to_string());
      AppError::InternalError(error.to_string())
   }
}

impl From<actix_web::Error> for AppError {
   fn from(error: actix_web::Error) -> Self {
      log::error!("{}", error.to_string());
      AppError::InternalError(error.to_string())
   }
}
