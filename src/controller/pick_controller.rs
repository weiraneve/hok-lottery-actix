use actix_web::{web, post, Responder};
use crate::model::post_param::PostParam;
use crate::service::pick_service::pick;

#[post("/")]
pub async fn pick_heroes(
    web::Json(param): web::Json<PostParam>,
    data: web::Data<sqlx::MySqlPool>,
) -> actix_web::Result<impl Responder> {
    let response_data = pick(&data, param).await?;
    Ok(web::Json(response_data))
}
