use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn auth200() -> impl Responder {
    HttpResponse::Forbidden()
}
async fn auth500() -> impl Responder {
    HttpResponse::BadRequest()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/auth").to(auth200))
            .default_service(web::route().to(auth500))
    })
        .bind(("0.0.0.0", 81))?
        .run()
        .await
}
