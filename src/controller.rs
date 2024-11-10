use crate::app::AppData;
use crate::error::AppError;
use crate::p;
use crate::responder::Html;
use actix_web::http::header;
use actix_web::web::Redirect;
use actix_web::Responder;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use tera::Context;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Position {
   x: usize,
   y: usize,
   width: usize,
   height: usize,
}

pub async fn get_new_game() -> impl Responder {
   let uuid = Uuid::new_v4().hyphenated().to_string();
   let url = format!("/game/{}", uuid);
   p!("BLZZZ");
   HttpResponse::Found()
      .append_header((header::LOCATION, url))
      .finish()
}

pub async fn get_game(
   id: web::Path<Uuid>,
   data: web::Data<AppData>,
) -> Result<impl Responder, AppError> {
   let id = id.into_inner();
   let context = Context::new();
   Ok(Html(data.tera.render("start.html", &context)?))
}

pub async fn post_position(
   json: web::Json<Position>,
) -> Result<impl Responder, AppError> {
   let pos = json.into_inner();
   p!(pos);
   Ok(HttpResponse::Ok())
}
