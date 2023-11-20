use actix_cors::Cors;
use std::{io, env};
use actix_web::{web, App, HttpServer, http};
use crate::controller::{pick_controller, reset_controller};

mod controller;
mod dao;
mod service;
mod model;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("DEBUG"));
    let server_addr = env::var("SERVER_ADDR").expect("SERVER_ADDR is not set in .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::MySqlPool::connect(&database_url).await.expect("Failed to create pool");
    log::info!("starting HTTP server at {server_addr}");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|_, _req_head| true)
            .allowed_methods(vec![http::Method::GET, http::Method::POST])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .service(pick_controller::pick_heroes)
            .service(reset_controller::clean_team)
            .service(reset_controller::clean_all_teams)
            .service(reset_controller::clean_all_heroes)
    })
        .bind(server_addr)?
        .run()
        .await
}
