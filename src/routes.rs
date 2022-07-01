use crate::handlers;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(handlers::index))
        .route("/hello", web::get().to(handlers::hello))
        .route("/echo", web::post().to(handlers::echo))
        .route("/ws", web::get().to(handlers::websocket))
        .route("/user/{id}/{name}", web::get().to(handlers::person));
}
