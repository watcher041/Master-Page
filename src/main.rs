
mod routes;
mod controllers;

use tera::Tera;
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .data( Tera::new("templates/**/*").unwrap() ) // teraの初期化
        .configure( routes::routing ) )
        .bind("localhost:3000")?
        .run()
        .await
}