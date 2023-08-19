use actix_web::{App, HttpServer, middleware};
use actix_web_httpauth::middleware::HttpAuthentication;

mod handlers;
mod models;
mod routes;
mod validator;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = ("0.0.0.0", 8888);

    println!("server running at {}:{}", addr.0, addr.1 );

    HttpServer::new(|| {
        let auth = HttpAuthentication::basic(validator::validator);
        App::new()
           .wrap(middleware::Logger::default())
           .wrap(auth)
           .configure(routes::routes)
    })
    .bind(addr)?
    .run()
    .await
}
