use actix_web::{web, get, Responder};
use crate::model::team_query::TeamQuery;
use crate::service::reset_service;
use crate::service::reset_service::reset_one_team;

#[get("/reset/team")]
pub async fn reset_team(
    query: web::Query<TeamQuery>,
    data: web::Data<sqlx::MySqlPool>,
) -> actix_web::Result<impl Responder> {
    let response_data = reset_one_team(query.id, &data).await?;
    Ok(web::Json(response_data))
}

#[get("/reset/teams")]
pub async fn reset_all_teams(data: web::Data<sqlx::MySqlPool>) -> actix_web::Result<impl Responder> {
    let response_data = reset_service::reset_all_teams(&data).await?;
    Ok(web::Json(response_data))
}

#[get("/reset/heroes")]
pub async fn reset_all_heroes(data: web::Data<sqlx::MySqlPool>) -> actix_web::Result<impl Responder> {
    let response_data = reset_service::reset_all_heroes(&data).await?;
    Ok(web::Json(response_data))
}
