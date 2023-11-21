use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, Error, http, web};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};

use crate::{AppState, controller};
use crate::dao::Database;

pub fn create_app(database: Arc<Database>) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response=ServiceResponse<impl MessageBody>,
        Error=Error,
        Config=(),
        InitError=()
    > + Sized
> {
    let app_state = web::Data::new(AppState { database });
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
