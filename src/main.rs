// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

extern crate boxygram_server;
use actix_files::Files;
use boxygram_server::app;
use boxygram_server::controller;
use boxygram_server::ws;
use std::collections::HashMap;

use actix_web::{middleware, web, App, HttpServer};
use std::io;
use std::sync::{Arc, Mutex};
use tera::Tera;

#[actix_web::main]
async fn main() -> io::Result<()> {
   dotenvy::dotenv().ok();

   let tera =
      Tera::new("web/**/*.html").expect("Failed to render templates");

   let config = app::AppConfig::new_from_env();

   env_logger::init_from_env(
      env_logger::Env::new().default_filter_or("info"),
   );
   log::info!(
      "starting server at http://{}:{}",
      config.bind_address,
      config.bind_port
   );

   let app_state = Arc::new(Mutex::new(app::AppState {
      channels: HashMap::new(),
      tera: tera,
   }));

   HttpServer::new(move || {
      App::new()
         .app_data(web::Data::new(app_state.clone()))
         .wrap(middleware::Logger::default())
         .wrap(middleware::Compress::default())
         .route(
            "/arena/new",
            web::get().to(controller::get_new_arena),
         )
         .route("/arena/{id}", web::get().to(controller::get_arena))
         .route("/arena/{id}/ws", web::get().to(ws::arena))
         .service(
            Files::new("/", config.static_path.clone())
               .index_file("index.html")
               .prefer_utf8(true),
         )
   })
   .bind((config.bind_address, config.bind_port))?
   .workers(1)
   .run()
   .await
}
