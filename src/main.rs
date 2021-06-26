use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/healthz", web::get().to(health_check))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}