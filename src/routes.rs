use actix_web::{get, Responder};

use crate::{
    models::{Hero, Log, Team},
    persistence::{

    }
};

#[get("/")]
pub(crate) async fn index() -> impl Responder {
    format!("Hello world")
}
