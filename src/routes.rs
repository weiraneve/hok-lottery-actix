use actix_web::{get, Responder};

#[get("/")]
pub(crate) async fn index() -> impl Responder {
    format!("Hello world")
}
