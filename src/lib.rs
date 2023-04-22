//!src/lib.rs

use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

// impl Responder is what we used at the start. As seen, it is not an explicit caller and will
// assume the type in the response
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

// Here, we explicitly mention the HttpResponse. There is no performance difference between the two
async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    format!("Hello {}!, {}!", &form.name, &form.email);
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
