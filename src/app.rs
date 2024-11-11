use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tera::{Context, Tera};
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
   pub tera: Tera,
}

pub type SharedAppState = Arc<Mutex<AppState>>;

pub fn build_static(tera: &Tera) {
   println!("Building static pages");
   let static_dir = Path::new("./web/static");
   let site_dir = static_dir.join("site");
   if site_dir.exists() {
      fs::remove_dir_all(site_dir).unwrap();
   }
   for tmpl in tera.get_template_names() {
      let path = Path::new(tmpl);
      if !path.starts_with("site") {
         continue;
      }
      let dir = static_dir.join(path.parent().unwrap());
      fs::create_dir_all(dir).unwrap();
      let context = Context::new();
      let file = fs::File::create(static_dir.join(tmpl)).unwrap();
      let res = tera.render_to(tmpl, &context, file);
      if res.is_err() {
         println!("failed to render: {}", tmpl);
      }
   }
}
