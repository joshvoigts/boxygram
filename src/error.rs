use actix_web::error;
use actix_web::http::StatusCode;
use thiserror::Error;
use tera;
use crate::p;

#[derive(Debug, Error, Clone)]
pub enum AppError {
   #[error("An internal error occurred. Please try again later.")]
   InternalError(String),
}

impl error::ResponseError for AppError {
   fn status_code(&self) -> StatusCode {
      match *self {
         AppError::InternalError(_) => {
            StatusCode::INTERNAL_SERVER_ERROR
         }
      }
   }
}

impl From<tera::Error> for AppError {
   fn from(error: tera::Error) -> Self {
      log::error!("{}", error.to_string());
      AppError::InternalError(error.to_string())
   }
}
