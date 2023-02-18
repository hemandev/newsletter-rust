use actix_web::{web, HttpRequest, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(_req: HttpRequest, form: web::Form<FormData>) -> impl Responder {
    format!("Welcome {}!", form.name);
    HttpResponse::Ok().finish()
}