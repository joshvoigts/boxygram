use crate::app::SharedAppState;
use crate::db;
use crate::error::AppError;
use crate::model::DrawingState;
use crate::model::DrawingStateDTO;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_ws::{handle, Message};
use futures_util::StreamExt;
use rusqlite::Connection;
use tokio::sync::broadcast;

fn incoming(arena_id: &str, text: &str) -> Result<String, AppError> {
   let mut conn = Connection::open("drawings.db")?;
   let trans = conn.transaction()?;

   let mut drawing_state =
      match db::load_full_state(&trans, &arena_id) {
         Ok(state) => state,
         Err(AppError::NotFound) => DrawingState::new(),
         err => err?,
      };
   let mut dto = serde_json::from_str::<DrawingStateDTO>(&text)?;
   if dto.event_type == "line_update" {
      if let Some(line) = dto.data.last() {
         drawing_state.add_line(line.clone());
      } else {
         return Err(AppError::InternalError(
            "Failed to add line".to_string(),
         ));
      }
   } else if dto.event_type == "full_state" {
      drawing_state = DrawingState::from_lines(dto.data);
   } else {
      return Err(AppError::InternalError(
         "Unknown event type".to_string(),
      ));
   }
   db::save_full_state(&trans, &arena_id, &drawing_state)?;
   trans.commit()?;
   dto = DrawingStateDTO {
      event_type: "full_state".to_string(),
      data: drawing_state.lines,
   };
   Ok(serde_json::to_string(&dto)?)
}

fn initial(arena_id: &str) -> Result<String, AppError> {
   let conn = Connection::open("drawings.db")?;

   let drawing_state = match db::load_full_state(&conn, &arena_id) {
      Ok(state) => state,
      Err(AppError::NotFound) => DrawingState::new(),
      err => err?,
   };
   let dto = DrawingStateDTO {
      event_type: "full_state".to_string(),
      data: drawing_state.lines,
   };
   Ok(serde_json::to_string(&dto)?)
}

pub async fn arena(
   req: HttpRequest,
   stream: web::Payload,
   path: web::Path<String>,
   app_state: web::Data<SharedAppState>,
) -> Result<HttpResponse, AppError> {
   let arena_id = path.into_inner();

   // Access or create a broadcast channel for the given arena_id
   let (tx, mut rx) = {
      let mut state = app_state.lock().unwrap();
      let sender = state
         .channels
         .entry(arena_id.clone())
         .or_insert_with(|| broadcast::channel(100).0)
         .clone();
      (sender.clone(), sender.subscribe())
   };

   // Call actix_ws::handle with the correct signature (req and
   // stream).
   let (response, mut session, mut msg_stream) =
      handle(&req, stream)?;

   match initial(&arena_id) {
      Ok(text) => {
         let _ = session.text(text.as_str()).await;
      }
      Err(err) => log::error!("{err}"),
   }

   // Spawn a task using actix_rt::spawn to handle incoming
   // messages from WebSocket clients.
   actix_rt::spawn(async move {
      while let Some(Ok(msg)) = msg_stream.next().await {
         if let Message::Text(text) = msg {
            let text = text.to_string();
            match incoming(&arena_id, &text) {
               Ok(dto_str) => {
                  let _ = tx.send(dto_str);
               }
               Err(err) => log::error!("{err}"),
            }
         }
      }
   });

   // Spawn a task to forward messages from the broadcast channel
   // to the WebSocket session.
   actix_rt::spawn(async move {
      while let Ok(msg) = rx.recv().await {
         // Send message to the WebSocket client
         let _ = session.text(msg).await;
      }
   });

   // Return the initial HTTP response to establish the WebSocket
   // connection.
   Ok(response)
}
