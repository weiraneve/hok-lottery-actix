use actix_web::Result;
use actix_web::web::Data;
use chrono::{FixedOffset, Utc};
use sqlx::MySqlPool;
use crate::dao::reset_dao;

use crate::dao::reset_dao::{find_team_by_id, reset_team};
use crate::model::my_result::MyResult;

pub async fn reset_one_team(id: i32, pool: &Data<MySqlPool>) -> Result<MyResult, actix_web::Error> {
    let mut result = MyResult {
        team_id: 0,
        data: "".to_string(),
        time: Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local(),
        logs: vec![],
    };
    match find_team_by_id(id, pool).await {
        Ok(()) => {
            reset_team(id, pool).await.expect("reset team failed");
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

pub async fn reset_all_teams(pool: &MySqlPool) -> Result<MyResult, actix_web::Error> {
    reset_dao::reset_all_teams(pool).await.expect("rest all teams failed");
    let result = MyResult {
        team_id: 0,
        data: "重置所有队伍成功".to_string(),
        time: Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local(),
        logs: vec![],
    };
    Ok(result)
}

pub async fn reset_all_heroes(pool: &MySqlPool) -> Result<MyResult, actix_web::Error> {
    reset_dao::reset_all_heroes(pool).await.expect("rest all heroes failed");
    let result = MyResult {
        team_id: 0,
        data: "重置所有英雄成功".to_string(),
        time: Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local(),
        logs: vec![],
    };
    Ok(result)
}
