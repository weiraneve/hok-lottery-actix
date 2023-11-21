use actix_web::{web, post, Responder};
use crate::AppState;
use crate::model::post_param::PostParam;
use crate::service::pick_service::pick;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(pick_heroes);
}

#[post("/")]
async fn pick_heroes(
    web::Json(param): web::Json<PostParam>,
    app_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let response_data = pick(param, &app_state).await?;
    Ok(web::Json(response_data))
}
