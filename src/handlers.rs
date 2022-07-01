use crate::models;

use actix::{Actor, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;

struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}

pub async fn hello() -> &'static str {
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

pub async fn websocket(req: HttpRequest , stream: web::Payload) -> impl Responder {
    let resp = ws::start(MyWs{}, &req, stream);
    resp
}
