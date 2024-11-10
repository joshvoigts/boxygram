use crate::controller;
use actix_web::web;

pub fn service_config(cfg: &mut web::ServiceConfig) {
   cfg.service(
      web::scope("/api/v1").service(
         web::resource("/pos")
            .route(web::post().to(controller::post_position)),
      ),
   )
   .service(
      web::resource("/game/new")
         .route(web::get().to(controller::get_new_game)),
   )
   .service(
      web::resource("/game/{id}")
         .route(web::get().to(controller::get_game)),
   );
}
