use std::{env, io};
use std::sync::Arc;

use actix_web::HttpServer;

use hok_lottery_actix::creat_app::create_app;
use hok_lottery_actix::dao::Database;

#[actix_web::main]
async fn main() -> io::Result<()> {
    init_environment();
    let server_addr = env::var("SERVER_ADDR").expect("SERVER_ADDR is not set in .env file");
    let database = init_database().await;

    log::info!("starting HTTP server at {server_addr}");

    HttpServer::new(move || { create_app(database.clone()) })
        .bind(server_addr)?
        .run()
        .await
}

fn init_environment() {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("DEBUG"));
}

async fn init_database() -> Arc<Database> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Arc::new(Database::new(&database_url).await)
}
