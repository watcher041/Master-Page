
mod routes;
mod controllers;

use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::routing) )
        .bind("localhost:3000")?
        .run()
        .await
}