use crate::handlers;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").to(handlers::index))
        .route("/echo", web::post().to(handlers::echo))
        .route("/user/{id}/{name}", web::get().to(handlers::person));
}
