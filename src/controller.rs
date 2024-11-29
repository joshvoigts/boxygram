use crate::db;
use crate::error::AppError;
use crate::responder::Html;
use actix_web::http::header;
use actix_web::HttpResponse;
use actix_web::Responder;
use rusqlite::Connection;
use uuid::Uuid;

pub async fn get_index() -> Result<impl Responder, AppError> {
   Ok(Html(include_str!("../web/index.html").to_string()))
}

pub async fn get_arena() -> Result<impl Responder, AppError> {
   Ok(Html(include_str!("../web/whiteboard.html").to_string()))
}

pub async fn get_new() -> Result<impl Responder, AppError> {
   let conn = Connection::open("drawings.db")?;

   let mut length = 4;
   let mut arena_id = create_arena_id(length);
   while db::load_full_state(&conn, &arena_id).is_ok() {
      length += 1;
      arena_id = create_arena_id(length);
   }
   let url = format!("/{}", arena_id);
   Ok(HttpResponse::Found()
      .append_header((header::LOCATION, url))
      .finish())
}

fn create_arena_id(length: usize) -> String {
   let uuid = Uuid::new_v4().hyphenated().to_string();
   return uuid[..length].to_string();
}
