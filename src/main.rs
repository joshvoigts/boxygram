// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

extern crate boxygram_server;
use actix_files::Files;
use boxygram_server::app;
use boxygram_server::route;

use actix_web::{middleware, web, App, HttpServer};
use std::io;
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

   let app_data = app::AppData { tera: tera };

   HttpServer::new(move || {
      App::new()
         .app_data(web::Data::new(app_data.clone()))
         .wrap(middleware::Logger::default())
         .wrap(middleware::Compress::default())
         .configure(route::service_config)
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
