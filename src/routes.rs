use crate::handlers;
use crate::validators;
use actix_web_httpauth::middleware::HttpAuthentication;

use actix_files as fs;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::basic(validators::validator);
    cfg.route("/", web::get().to(handlers::index))
        .route("/hello", web::get().to(handlers::hello))
        .route("/echo", web::post().to(handlers::echo))
        .route("/ws", web::get().to(handlers::websocket))
        .route("/user/{id}/{name}", web::get().to(handlers::person))
        .service(
            web::scope("/secret")
                .wrap(auth)
                .service(fs::Files::new("/", "./static/secret").index_file("index.html")),
        );
}
