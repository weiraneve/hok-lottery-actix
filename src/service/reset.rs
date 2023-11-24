use std::sync::Arc;

use async_trait::async_trait;
use chrono::{FixedOffset, NaiveDateTime, Utc};

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
    ) -> Self {
        ResetServiceImpl { hero_repository, team_repository }
    }

    fn create_result(data: String) -> MyResult {
        MyResult {
            team_id: 0,
            data,
            time: current_time(),
            logs: vec![],
        }
    }
}

#[async_trait]
impl ResetService for ResetServiceImpl {
    async fn reset_one_team(&self, id: i32) -> Result<MyResult, actix_web::Error> {
        match self.team_repository.get_by_id(id).await {
            Ok(()) => {
                self.team_repository.reset_one(id).await.expect("reset one team failed");
                Ok(Self::create_result(format!("刷新队伍{}成功", id)))
            }
            Err(e) => {
                let message = match e {
                    sqlx::Error::RowNotFound => "未有查询到此队伍".to_string(),
                    _ => format!("处理时遇到错误：{}", e),
                };
                Ok(Self::create_result(message))
            }
        }
    }

    async fn reset_all_teams(&self) -> Result<MyResult, actix_web::Error> {
        self.team_repository.reset_all().await.expect("reset all teams failed");
        Ok(Self::create_result("重置所有队伍成功".to_string()))
    }

    async fn reset_all_heroes(&self) -> Result<MyResult, actix_web::Error> {
        self.hero_repository.reset().await.expect("reset all heroes failed");
        Ok(Self::create_result("重置所有英雄成功".to_string()))
    }
}

fn current_time() -> NaiveDateTime {
    Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local()
}
