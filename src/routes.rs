use actix_web::{web, get, post, Responder};
use serde::Deserialize;
use crate::{
    models::{PostParam},
    persistence::{pick, clear_one_team, clear_all_teams, clear_all_heroes},
};

#[post("/")]
pub async fn pick_heroes(
    web::Json(param): web::Json<PostParam>,
    data: web::Data<sqlx::MySqlPool>,
) -> actix_web::Result<impl Responder> {
    let response_data = pick(&data, param).await?;
    Ok(web::Json(response_data))
}

#[get("/reset/team")]
pub async fn clean_team(
    query: web::Query<TeamQuery>,
    data: web::Data<sqlx::MySqlPool>
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

#[derive(Deserialize)]
pub struct TeamQuery {
    id: i32,
}
