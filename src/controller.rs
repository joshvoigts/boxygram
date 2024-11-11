use crate::app::SharedAppState;
use crate::error::AppError;
use crate::responder::Html;
use actix_web::http::header;
use actix_web::Responder;
use actix_web::{web, HttpResponse};
use tera::Context;
use uuid::Uuid;

pub async fn get_new_arena() -> impl Responder {
   let uuid = Uuid::new_v4().hyphenated().to_string();
   let url = format!("/arena/{}", uuid);
   HttpResponse::Found()
      .append_header((header::LOCATION, url))
      .finish()
}

pub async fn get_arena(
   data: web::Data<SharedAppState>,
) -> Result<impl Responder, AppError> {
   let context = Context::new();
   Ok(Html(
      data
         .lock()
         .unwrap()
         .tera
         .render("whiteboard.html", &context)?,
   ))
}
