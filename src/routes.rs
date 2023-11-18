use actix_web::{web, post, Responder};
use crate::{
    models::{PostParam},
    persistence::{pick},
};

#[post("/")]
pub async fn pick_heroes(
    web::Json(param): web::Json<PostParam>,
    data: web::Data<sqlx::MySqlPool>,
) -> actix_web::Result<impl Responder> {
    let response_data = pick(&data, param).await?;
    Ok(web::Json(response_data))
}
