use actix_web::{middleware, App, HttpServer};

mod handlers;
mod models;
mod routes;
mod validators;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = ("0.0.0.0", 8888);

    println!("server running at {}:{}", addr.0, addr.1);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(routes::routes)
    })
    .bind(addr)?
    .run()
    .await
}
