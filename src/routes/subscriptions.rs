//! src/routes/subscriptions.rs

use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// Here, we explicitly mention the HttpResponse. There is no performance difference between the two
pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    format!("Hello {}!, {}!", &form.name, &form.email);
    HttpResponse::Ok().finish()
}
