use std::sync::Arc;

use async_trait::async_trait;
use chrono::{FixedOffset, Utc};

use crate::model::hero::Hero;
use crate::model::log_response::Log;
use crate::model::my_result::MyResult;
use crate::model::post_param::PostParam;
use crate::model::team::Team;
use crate::repository::hero::HeroRepository;
use crate::repository::log::LogRepository;
use crate::repository::team::TeamRepository;

#[async_trait]
pub trait PickService: Sync + Send {
    async fn pick(&self, param: PostParam) -> Result<MyResult, actix_web::Error>;
}

pub struct PickServiceImpl {
    pub hero_repository: Arc<dyn HeroRepository>,
    pub team_repository: Arc<dyn TeamRepository>,
    pub log_repository: Arc<dyn LogRepository>,
}

impl PickServiceImpl {
    pub fn new(
        hero_repository: Arc<dyn HeroRepository>,
        team_repository: Arc<dyn TeamRepository>,
        log_repository: Arc<dyn LogRepository>,
    ) -> Self { PickServiceImpl { hero_repository, team_repository, log_repository } }
}

#[async_trait]
impl PickService for PickServiceImpl {
    async fn pick(&self, param: PostParam) -> Result<MyResult, actix_web::Error> {
        match self.team_repository.get_team_by_encrypt_code(param.encrypt_code).await {
            Ok(mut team) => {
                let team = &mut team;
                let mut result = MyResult {
                    team_id: team.id,
                    data: team.clone().pick_content,
                    time: Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local(),
                    logs: self.team_repository.get_log_by_team_id(team.id).await.unwrap(),
                };
                self.check_team_is_picked(team, &mut result).await;
                Ok(result)
            }
            Err(e) => Err(actix_web::error::ErrorInternalServerError(e))
        }
    }
}

impl PickServiceImpl {
    async fn check_team_is_picked(&self, team: &mut Team, result: &mut MyResult) {
        if !team.is_picked {
            let pick_result = &format!("{}or{}", self.pick_hero().await, self.pick_hero().await);
            result.data = pick_result.clone();
            self.save_result_for_log(team.id, pick_result).await;
            self.update_team_is_picked(team, pick_result).await;
        }
    }

    async fn update_team_is_picked(&self, team: &mut Team, pick_result: &String) {
        team.is_picked = true;
        team.pick_content = pick_result.clone();
        team.update_time = Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local();
        self.team_repository.save_team(team.clone()).await.expect("save team failed");
    }

    async fn save_result_for_log(&self, team_index: i32, pick_result: &String) {
        let log = Log {
            team_id: team_index,
            pick_group: pick_result.clone(),
            time: Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local(),
        };
        self.log_repository.save_log(log).await.expect("save log failed");
    }

    async fn pick_hero(&self) -> String {
        self.get_second_random_hero(self.get_first_random_hero().await).await
    }

    async fn get_first_random_hero(&self) -> Hero {
        let mut hero = self.hero_repository.get_not_is_pick().await.unwrap();
        self.save_hero_and_is_pick(&mut hero).await.expect("save hero failed");
        return hero;
    }

    async fn get_second_random_hero(&self, exist_hero: Hero) -> String {
        let hero = self.hero_repository.get_not_is_pick().await.unwrap();
        format!("[{}][{}]", exist_hero.name, hero.name)
    }

    async fn save_hero_and_is_pick(&self, hero: &mut Hero) -> Result<(), sqlx::Error> {
        hero.is_pick = true;
        self.hero_repository.save(hero.clone()).await
    }
}
