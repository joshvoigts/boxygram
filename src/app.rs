use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

pub struct AppConfig {
   pub bind_address: String,
   pub bind_port: u16,
   pub environment: String,
   pub static_path: String,
}

impl AppConfig {
   pub fn new_from_env() -> Self {
      let bind_address = std::env::var("BOXY_BIND_ADDRESS")
         .unwrap_or("127.0.0.1".to_string());
      let bind_port: u16 = std::env::var("BOXY_BIND_PORT")
         .unwrap_or("8080".to_string())
         .parse()
         .expect("BOXY_BIND_PORT must be a u16");
      let environment = std::env::var("BOXY_ENV")
         .unwrap_or("development".to_string());
      let static_path = std::env::var("BOXY_STATIC_PATH")
         .unwrap_or("web".to_string());

      AppConfig {
         bind_address: bind_address,
         bind_port: bind_port,
         environment: environment,
         static_path: static_path,
      }
   }
}

#[derive(Clone)]
pub struct AppState {
   pub channels: HashMap<String, broadcast::Sender<String>>,
}

pub type SharedAppState = Arc<Mutex<AppState>>;
