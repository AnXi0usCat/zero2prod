use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

async fn health_check(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
