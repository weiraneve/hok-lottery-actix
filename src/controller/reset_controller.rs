use actix_web::{web, get, Responder};
use crate::model::team_query::TeamQuery;
use crate::service::reset_service::{clear_all_heroes, clear_all_teams, clear_one_team};

#[get("/reset/team")]
pub async fn clean_team(
    query: web::Query<TeamQuery>,
    data: web::Data<sqlx::MySqlPool>,
) -> actix_web::Result<impl Responder> {
    let response_data = clear_one_team(query.id, &data).await?;
    Ok(web::Json(response_data))
}

#[get("/reset/teams")]
pub async fn clean_all_teams(data: web::Data<sqlx::MySqlPool>) -> actix_web::Result<impl Responder> {
    let response_data = clear_all_teams(&data).await?;
    Ok(web::Json(response_data))
}

#[get("/reset/heroes")]
pub async fn clean_all_heroes(data: web::Data<sqlx::MySqlPool>) -> actix_web::Result<impl Responder> {
    let response_data = clear_all_heroes(&data).await?;
    Ok(web::Json(response_data))
}
