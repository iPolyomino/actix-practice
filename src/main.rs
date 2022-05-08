use actix_web::{App, HttpServer, middleware};

mod handlers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = ("127.0.0.1", 8080);

    println!("server running at {}:{}", addr.0, addr.1 );

    HttpServer::new(|| {
        App::new()
           .wrap(middleware::Logger::default())
           .configure(routes::routes)
    })
    .bind(addr)?
    .run()
    .await
}
