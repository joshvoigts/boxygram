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
         .expect("BOXY_BIND_ADDRESS must be set");
      let bind_port: u16 = std::env::var("BOXY_BIND_PORT")
         .expect("BOXY_BIND_PORT must be set")
         .parse()
         .expect("BOXY_BIND_PORT must be a u16");
      let environment =
         std::env::var("BOXY_ENV").expect("BOXY_ENV must be set");
      let static_path = std::env::var("BOXY_STATIC_PATH")
         .expect("BOXY_STATIC_PATH must be set");

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
