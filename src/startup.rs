//! src/startup.rs

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use crate::routes::{greet, subscribe, health_check};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/routes/health_check", web::get().to(health_check))
            .route("/routes/subscriptions", web::post().to(subscribe))
            .route("/routes/", web::get().to(greet))
            .route("/routes/{name}", web::get().to(greet))
    })
        .listen(listener)?
        .run();
    Ok(server)
}