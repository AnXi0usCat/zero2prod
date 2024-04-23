use actix_web::{web, HttpResponse};
use sqlx::PgConnection;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(_form: web::Form<FormData>, _connection: web::Data<PgConnection>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
