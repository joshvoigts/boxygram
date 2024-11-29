extern crate boxygram;
use actix_files::Files;
use boxygram::{app, controller, ws};
use std::collections::HashMap;

use actix_web::{middleware, web, App, HttpServer};
use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> io::Result<()> {
   dotenvy::dotenv().ok();

   let config = app::AppConfig::new_from_env();

   env_logger::Builder::new()
      .format(|buf, record| {
         writeln!(
            buf,
            "{}:{} {} [{}] {}",
            record.file().unwrap_or("unknown"),
            record.line().unwrap_or(0),
            chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
            record.level(),
            record.args()
         )
      })
      .init();

   log::info!(
      "starting server at http://{}:{}",
      config.bind_address,
      config.bind_port
   );

   let app_state = Arc::new(Mutex::new(app::AppState {
      channels: HashMap::new(),
   }));

   HttpServer::new(move || {
      App::new()
         .app_data(web::Data::new(app_state.clone()))
         .wrap(middleware::Logger::default())
         .wrap(middleware::Compress::default())
         .route("/", web::get().to(controller::get_index))
         .route("/new", web::get().to(controller::get_new))
         .route("/{id}", web::get().to(controller::get_arena))
         .route("/{id}/ws", web::get().to(ws::arena))
         .service(
            Files::new("/static", config.static_path.clone())
               .prefer_utf8(true),
         )
   })
   .bind((config.bind_address, config.bind_port))?
   .workers(1)
   .run()
   .await
}
