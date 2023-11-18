use crate::{models::{PostParam, MyResult, Team}};
use actix_web::{Result};
use chrono::Utc;
use crate::models::LogResponse;

pub async fn pick(pool: &sqlx::MySqlPool, param: PostParam) -> Result<MyResult, actix_web::Error> {
    if let Ok(team) = get_team_by_encrypt_code(param.encrypt_code, pool).await {
        let log_responses = get_log_by_team_id(team.id, pool).await.unwrap();
        Ok(MyResult {
            team_id: team.id,
            data: team.pick_content,
            time: Utc::now().naive_utc(),
            logs: log_responses,
        })
    } else {
        Err(actix_web::error::ErrorNotFound("No team found with the given encrypt code"))
    }
}


async fn get_team_by_encrypt_code(encrypt_code: String, pool: &sqlx::MySqlPool) -> Result<Team, sqlx::Error> {
    sqlx::query_as::<_, Team>("SELECT * FROM team WHERE encrypt_code = ?")
        .bind(encrypt_code)
        .fetch_one(pool)
        .await
}

async fn get_log_by_team_id(team_id: i32, pool: &sqlx::MySqlPool) -> Result<Vec<LogResponse>, sqlx::Error> {
    sqlx::query_as::<_, LogResponse>("SELECT * FROM log l WHERE l.team_id = ? ORDER BY l.time DESC")
        .bind(team_id.to_string())
        .fetch_all(pool)
        .await
}
