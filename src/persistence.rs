use crate::{models::{PostParam, MyResult, Team, Hero}};
use actix_web::{Result};
use chrono::{FixedOffset, Utc};
use sqlx::{Error, MySqlPool};
use crate::models::LogResponse;

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

async fn save_hero_and_is_pick(hero: &mut Hero, pool: &MySqlPool) -> Result<(), Error> {
    hero.is_pick = true;
    save_hero(hero.clone(), pool).await
}

async fn get_team_by_encrypt_code(encrypt_code: String, pool: &MySqlPool) -> Result<Team, Error> {
    sqlx::query_as::<_, Team>("SELECT * FROM `team` WHERE `encrypt_code` = ?")
        .bind(encrypt_code)
        .fetch_one(pool)
        .await
}

async fn get_log_by_team_id(team_id: i32, pool: &MySqlPool) -> Result<Vec<LogResponse>, Error> {
    sqlx::query_as::<_, LogResponse>("SELECT * FROM `log` l WHERE l.team_id = ? ORDER BY l.time DESC")
        .bind(team_id.to_string())
        .fetch_all(pool)
        .await
}

async fn get_heroes_not_is_pick(pool: &MySqlPool) -> Result<Hero, Error> {
    sqlx::query_as::<_, Hero>("SELECT * FROM `hero` h WHERE h.is_pick = FALSE ORDER BY RAND() LIMIT 1")
        .fetch_one(pool)
        .await
}

async fn save_hero(hero: Hero, pool: &MySqlPool) -> std::result::Result<(), Error> {
    sqlx::query(r#"INSERT INTO hero (id, name, line, is_pick)
     VALUES (?, ?, ?, ?)
      ON DUPLICATE KEY UPDATE
       name = VALUES(name),
        line = VALUES(line),
         is_pick = VALUES(is_pick)
    "#)
        .bind(hero.id)
        .bind(&hero.name)
        .bind(&hero.line)
        .bind(hero.is_pick)
        .execute(pool)
        .await?;

    Ok(())
}

async fn save_log(log: LogResponse, pool: &MySqlPool) -> std::result::Result<(), Error> {
    sqlx::query("INSERT INTO `log` (`team_id`, `pick_group`, `time`) VALUES (?, ?, ?)")
        .bind(log.team_id)
        .bind(log.pick_group)
        .bind(log.time)
        .execute(pool)
        .await?;

    Ok(())
}

async fn save_team(team: Team, pool: &MySqlPool) -> std::result::Result<(), Error> {
    sqlx::query(r#"
        INSERT INTO `team` (`id`, `encrypt_code`, `pick_content`, `is_picked`, `update_time`)
        VALUES (?, ?, ?, ?, ?)
        ON DUPLICATE KEY UPDATE
            `encrypt_code` = VALUES(`encrypt_code`),
            `pick_content` = VALUES(`pick_content`),
            `is_picked` = VALUES(`is_picked`),
            `update_time` = VALUES(`update_time`)
    "#)
        .bind(team.id)
        .bind(team.encrypt_code)
        .bind(team.pick_content)
        .bind(team.is_picked)
        .bind(team.update_time)
        .execute(pool)
        .await?;

    Ok(())
}
