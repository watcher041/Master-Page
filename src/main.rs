
use actix_web::{web, App, HttpServer, Responder};

async fn home() -> impl Responder {
    "Hello,world!"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .route("/", web::get().to(home)))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}