use std::{env, io};
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, http, HttpServer, web};

use hok_lottery_actix::{AppState, controller};
use hok_lottery_actix::dao::Database;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("DEBUG"));
    let server_addr = env::var("SERVER_ADDR").expect("SERVER_ADDR is not set in .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    log::info!("starting HTTP server at {server_addr}");

    let database = Database::new(&database_url).await;
    let app_state = web::Data::new(AppState { database: Arc::new(database) });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|_, _req_head| true)
            .allowed_methods(vec![http::Method::GET, http::Method::POST])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .configure(controller::init_pick_controller)
            .configure(controller::init_reset_controller)
    })
        .bind(server_addr)?
        .run()
        .await
}
