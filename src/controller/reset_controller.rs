use actix_web::{get, Responder, web};

use crate::AppState;
use crate::model::team_query::TeamQuery;
use crate::service::reset_service;
use crate::service::reset_service::reset_one_team;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(reset_team);
    cfg.service(reset_all_teams);
    cfg.service(reset_all_heroes);
}

#[get("/reset/team")]
pub async fn reset_team(
    query: web::Query<TeamQuery>,
    app_state: web::Data<AppState<'_>>,
) -> actix_web::Result<impl Responder> {
    let response_data = reset_one_team(query.id, &app_state).await?;
    Ok(web::Json(response_data))
}

#[get("/reset/teams")]
pub async fn reset_all_teams(app_state: web::Data<AppState<'_>>) -> actix_web::Result<impl Responder> {
    let response_data = reset_service::reset_all_teams(&app_state).await?;
    Ok(web::Json(response_data))
}

#[get("/reset/heroes")]
pub async fn reset_all_heroes(app_state: web::Data<AppState<'_>>) -> actix_web::Result<impl Responder> {
    let response_data = reset_service::reset_all_heroes(&app_state).await?;
    Ok(web::Json(response_data))
}
