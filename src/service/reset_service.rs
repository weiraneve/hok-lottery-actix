use actix_web::Result;
use actix_web::web::Data;
use chrono::{FixedOffset, Utc};
use crate::app_state::AppState;

use crate::model::my_result::MyResult;

pub async fn reset_one_team(id: i32, app_state: &Data<AppState>) -> Result<MyResult, actix_web::Error> {
    let mut result = MyResult {
        team_id: 0,
        data: "".to_string(),
        time: Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local(),
        logs: vec![],
    };
    match app_state.database.teams.find_team_by_id(id).await {
        Ok(()) => {
            app_state.database.teams.reset_team(id).await.expect("reset team failed");
            result.data = format!("刷新队伍{}成功", id);
        }
        Err(e) => {
            match e {
                sqlx::Error::RowNotFound => {
                    result.data = "未有查询到此队伍".to_string();
                }
                _ => {
                    result.data = format!("处理时遇到错误：{}", e);
                }
            }
        }
    };
    Ok(result)
}

pub async fn reset_all_teams(app_state: &Data<AppState>) -> Result<MyResult, actix_web::Error> {
    app_state.database.teams.reset_all_teams().await.map_err(actix_web::error::ErrorInternalServerError)?;
    let result = MyResult {
        team_id: 0,
        data: "重置所有队伍成功".to_string(),
        time: Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local(),
        logs: vec![],
    };
    Ok(result)
}

pub async fn reset_all_heroes(app_state: &Data<AppState>) -> Result<MyResult, actix_web::Error> {
    app_state.database.heroes.reset_all_heroes().await.map_err(actix_web::error::ErrorInternalServerError)?;
    let result = MyResult {
        team_id: 0,
        data: "重置所有英雄成功".to_string(),
        time: Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local(),
        logs: vec![],
    };
    Ok(result)
}
