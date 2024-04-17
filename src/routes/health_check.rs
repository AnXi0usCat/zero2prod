use actix_web::{HttpRequest, HttpResponse};

pub async fn health_check(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}
