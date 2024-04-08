use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer};

async fn health_check(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
