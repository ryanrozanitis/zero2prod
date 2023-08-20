//! tests/health_check.rs

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)

use actix_web::{HttpResponse, Responder};

// impl Responder is what we used at the start. As seen, it is not an explicit caller and will
// assume the type in the response
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
