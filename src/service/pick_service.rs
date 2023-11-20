use actix_web::Result;
use chrono::{FixedOffset, Utc};
use sqlx::{MySqlPool};
use crate::dao::pick_dao::{get_heroes_not_is_pick, get_log_by_team_id, get_team_by_encrypt_code, save_hero, save_log, save_team};
use crate::model::{
    hero::Hero,
    log_response::LogResponse,
    my_result::MyResult,
    post_param::PostParam,
    team::Team,
};

pub async fn pick(pool: &MySqlPool, param: PostParam) -> Result<MyResult, actix_web::Error> {
    if let Ok(mut team) = get_team_by_encrypt_code(param.encrypt_code, pool).await {
        let team = &mut team;
        let mut result = MyResult {
            team_id: team.id,
            data: team.clone().pick_content,
            time: Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local(),
            logs: get_log_by_team_id(team.id, pool).await.unwrap(),
        };
        check_team_is_picked(team, &mut result, pool).await;
        Ok(result)
    } else {
        Err(actix_web::error::ErrorNotFound("No team found with the given encrypt code"))
    }
}

async fn check_team_is_picked(team: &mut Team, result: &mut MyResult, pool: &MySqlPool) {
    if !team.is_picked {
        let pick_result = &format!("{}or{}", pick_hero(pool).await, pick_hero(pool).await);
        result.data = pick_result.clone();
        save_result_for_log(team.id, pick_result, pool).await;
        update_team_is_picked(team, pick_result, pool).await;
    }
}

async fn update_team_is_picked(team: &mut Team, pick_result: &String, pool: &MySqlPool) {
    team.is_picked = true;
    team.pick_content = pick_result.clone();
    team.update_time = Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local();
    save_team(team.clone(), pool).await.expect("save team failed");
}

async fn save_result_for_log(team_index: i32, pick_result: &String, pool: &MySqlPool) {
    let log = LogResponse {
        team_id: team_index,
        pick_group: pick_result.clone(),
        time: Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local(),
    };
    save_log(log, pool).await.expect("save log failed");
}

async fn pick_hero(pool: &MySqlPool) -> String {
    get_second_random_hero(get_first_random_hero(pool).await, pool).await
}

async fn get_first_random_hero(pool: &MySqlPool) -> Hero {
    let mut hero = get_heroes_not_is_pick(pool).await.unwrap();
    save_hero_and_is_pick(&mut hero, pool).await.expect("save hero failed");
    return hero;
}

async fn get_second_random_hero(exist_hero: Hero, pool: &MySqlPool) -> String {
    let hero = get_heroes_not_is_pick(pool).await.unwrap();
    format!("[{}][{}]", exist_hero.name, hero.name)
}

async fn save_hero_and_is_pick(hero: &mut Hero, pool: &MySqlPool) -> Result<(), sqlx::Error> {
    hero.is_pick = true;
    save_hero(hero.clone(), pool).await
}


