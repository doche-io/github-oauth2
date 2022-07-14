use actix_redis::RedisSession;
use actix_session::Session;
use actix_web::{http::StatusCode, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

async fn auth200() -> impl Responder {
    "".customize()
        .with_status(StatusCode::from_u16(200).unwrap())
}
async fn auth401() -> impl Responder {
    "".customize()
        .with_status(StatusCode::from_u16(401).unwrap())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/auth").to(auth200))
            .default_service(web::route().to(auth401))
    })
    .bind(("0.0.0.0", 81))?
    .run()
    .await
}
