use actix_web::{web, post, HttpResponse, Responder};

use crate::{
    models::{Hero, Log, Team, MyResult, PostParam},
    persistence::{pick},
};

#[post("/")]
pub async fn pick_heroes(
    web::Json(param): web::Json<PostParam>,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    let response_data = web::block(move || pick(param, &data)).await??;
    Ok(web::Json(response_data))
}
