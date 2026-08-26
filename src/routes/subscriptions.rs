use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormDate {
    email: String,
    name: String
}

pub async fn subscribe(_form: web::Form<FormDate>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
