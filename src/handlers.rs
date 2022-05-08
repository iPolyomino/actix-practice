use crate::models;
use actix_web::{web, Responder, HttpResponse};

pub async fn index() -> &'static str {
    "Hello world!"
}

pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn person(user: web::Path<models::User>) -> impl Responder {
    let id = user.id;
    let name = user.name.clone();
    HttpResponse::Ok().json(models::User { id, name })
}
