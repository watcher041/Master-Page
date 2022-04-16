
mod routes;
mod controllers;

use tera::Tera;
use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(Files::new("/assets", ".").show_files_listing())
        .data( Tera::new("src/templates/**/*").unwrap() ) // teraの初期化
        .configure( routes::routing ) 
    )
    .bind("localhost:3000")?
    .run()
    .await
}