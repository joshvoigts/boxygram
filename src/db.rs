use crate::error::AppError;
use crate::model::DrawingState;
use rusqlite::params;
use rusqlite::Connection;
use serde_json;

// pub fn get_trans() -> Result<Connection, AppError> {
//    let mut conn = Connection::open("drawings.db")?;
//    Ok(conn.transaction()?)
// }

pub fn save_full_state(
   conn: &Connection,
   arena_id: &str,
   drawing_state: &DrawingState,
) -> Result<(), AppError> {
   let drawing_state_lines =
      serde_json::to_string(&drawing_state.lines)?;

   conn.execute(
      r#"
         INSERT OR REPLACE INTO drawing_state
         (arena_id, lines, modified)
         VALUES
         (?1, ?2, ?3)
      "#,
      params![arena_id, drawing_state_lines, &drawing_state.modified],
   )?;
   Ok(())
}

pub fn load_full_state(
   conn: &Connection,
   arena_id: &str,
) -> Result<DrawingState, AppError> {
   let mut stmt = conn.prepare(
      r#"
         SELECT lines, modified
         FROM drawing_state
         WHERE arena_id = ?1
      "#,
   )?;
   let (lines_json, modified) =
      stmt.query_row(params![arena_id], |row| {
         let lines_json: String = row.get(0)?;
         let modified: String = row.get(1)?;

         Ok((lines_json, modified))
      })?;

   // Deserialize the JSON string into a DrawingState struct
   let drawing_state = DrawingState {
      lines: serde_json::from_str(&lines_json)?,
      modified: modified,
   };

   Ok(drawing_state)
}
