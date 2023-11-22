use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, Error, http, web};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use sqlx::MySqlPool;

use crate::controller;
use crate::app_state::AppState;
use crate::dao::Database;
use crate::repository::hero::HeroRepositoryImpl;
use crate::repository::log::LogRepositoryImpl;
use crate::repository::team::TeamRepositoryImpl;
use crate::service::pick::PickServiceImpl;

pub fn create_app(database: Arc<Database>, pool: Arc<MySqlPool>) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response=ServiceResponse<impl MessageBody>,
        Error=Error,
        Config=(),
        InitError=()
    > + Sized
> {
    let pick_service = Arc::new(PickServiceImpl::new(
        Arc::new(HeroRepositoryImpl::new(pool.clone())),
        Arc::new(TeamRepositoryImpl::new(pool.clone())),
        Arc::new(LogRepositoryImpl::new(pool.clone())),
    ));
    let app_state = web::Data::new(AppState { database, pick_service });
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
}
