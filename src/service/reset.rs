use std::sync::Arc;

use async_trait::async_trait;
use chrono::{FixedOffset, Utc};

use crate::model::my_result::MyResult;
use crate::repository::hero::HeroRepository;
use crate::repository::team::TeamRepository;

#[async_trait]
pub trait ResetService: Sync + Send {
    async fn reset_one_team(&self, id: i32) -> Result<MyResult, actix_web::Error>;
    async fn reset_all_teams(&self) -> Result<MyResult, actix_web::Error>;
    async fn reset_all_heroes(&self) -> Result<MyResult, actix_web::Error>;
}

pub struct ResetServiceImpl {
    pub hero_repository: Arc<dyn HeroRepository>,
    pub team_repository: Arc<dyn TeamRepository>,
}

impl ResetServiceImpl {
    pub fn new(
        hero_repository: Arc<dyn HeroRepository>,
        team_repository: Arc<dyn TeamRepository>,
    ) -> Self { ResetServiceImpl { hero_repository, team_repository } }
}

#[async_trait]
impl ResetService for ResetServiceImpl {
    async fn reset_one_team(&self, id: i32) -> Result<MyResult, actix_web::Error> {
        let mut result = MyResult {
            team_id: 0,
            data: "".to_string(),
            time: Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local(),
            logs: vec![],
        };
        match self.team_repository.get_by_id(id).await {
            Ok(()) => {
                self.team_repository.reset_one(id).await.expect("reset team failed");
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

    async fn reset_all_teams(&self) -> Result<MyResult, actix_web::Error> {
        self.team_repository.reset_all().await.map_err(actix_web::error::ErrorInternalServerError)?;
        let result = MyResult {
            team_id: 0,
            data: "重置所有队伍成功".to_string(),
            time: Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local(),
            logs: vec![],
        };
        Ok(result)
    }

    async fn reset_all_heroes(&self) -> Result<MyResult, actix_web::Error> {
        self.hero_repository.reset().await.map_err(actix_web::error::ErrorInternalServerError)?;
        let result = MyResult {
            team_id: 0,
            data: "重置所有英雄成功".to_string(),
            time: Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local(),
            logs: vec![],
        };
        Ok(result)
    }
}
