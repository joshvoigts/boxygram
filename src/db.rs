use crate::error::AppError;
use crate::model::DrawingState;
use rusqlite::{params, Connection};
use serde_json;

pub fn save_full_state(
   arena_id: &str,
   drawing_state: &DrawingState,
) -> Result<(), AppError> {
   let conn = Connection::open("drawings.db")?;

   let drawing_state_json = serde_json::to_string(drawing_state)?;

   conn.execute(
        "INSERT OR REPLACE INTO drawing_state (arena_id, lines, modified) VALUES (?1, ?2, ?3)",
        params![arena_id, drawing_state_json, &drawing_state.modified],
    )?;
   Ok(())
}

pub fn load_full_state(
   arena_id: &str,
) -> Result<DrawingState, AppError> {
   let conn = Connection::open("drawings.db")?;

   let mut stmt = conn.prepare(
      "SELECT lines, modified FROM drawing_state WHERE arena_id = ?1",
   )?;
   let (lines_json, modified) =
      stmt.query_row(params![arena_id], |row| {
         let lines_json: String = row.get(0)?;
         let modified: String = row.get(1)?;

         Ok((lines_json, modified))
      })?;

   // Deserialize the JSON string into a DrawingState struct
   let drawing_state: DrawingState =
      serde_json::from_str(&lines_json)?;
   dbg!(&drawing_state);

   Ok(drawing_state)
}
