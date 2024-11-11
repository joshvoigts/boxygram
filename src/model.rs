use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Point {
   pub x: f64,
   pub y: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Line(Vec<Point>);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DrawingStateDTO {
   pub event_type: String,
   pub data: Vec<Line>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DrawingState {
   pub lines: Vec<Line>,
   pub modified: String, // stores the timestamp of the last modification
}

impl DrawingState {
   // Helper function to create a new DrawingState with current timestamp
   pub fn new() -> Self {
      DrawingState {
         lines: Vec::new(),
         modified: Utc::now().to_rfc3339(),
      }
   }
}