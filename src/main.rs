use std::{io, env};
use actix_web::{App, HttpServer};

mod persistence;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("setting up app from environment");

    let server_addr = env::var("SERVER_ADDR").expect("SERVER_ADDR is not set in .env file");

    log::info!("starting HTTP server at {server_addr}");

    HttpServer::new(move || {
        App::new()
            .service(persistence::index)
    })
        .bind(server_addr)?
        .run()
        .await
}
