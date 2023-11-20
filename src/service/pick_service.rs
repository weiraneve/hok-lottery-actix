use actix_web::Result;
use actix_web::web::Data;
use chrono::{FixedOffset, Utc};
use crate::AppState;
use crate::model::{
    hero::Hero,
    log_response::LogResponse,
    my_result::MyResult,
    post_param::PostParam,
    team::Team,
};

pub async fn pick(param: PostParam, app_state: &Data<AppState<'_>>) -> Result<MyResult, actix_web::Error> {
    match app_state.context.teams.get_team_by_encrypt_code(param.encrypt_code).await {
        Ok(mut team) => {
            let team = &mut team;
            let mut result = MyResult {
                team_id: team.id,
                data: team.clone().pick_content,
                time: Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local(),
                logs: app_state.context.teams.get_log_by_team_id(team.id).await.unwrap(),
            };
            check_team_is_picked(team, &mut result, app_state).await;
            Ok(result)
        }
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e))
    }
}

async fn check_team_is_picked(team: &mut Team, result: &mut MyResult, app_state: &Data<AppState<'_>>) {
    if !team.is_picked {
        let pick_result = &format!("{}or{}", pick_hero(app_state).await, pick_hero(app_state).await);
        result.data = pick_result.clone();
        save_result_for_log(team.id, pick_result, app_state).await;
        update_team_is_picked(team, pick_result, app_state).await;
    }
}

async fn update_team_is_picked(team: &mut Team, pick_result: &String, app_state: &Data<AppState<'_>>) {
    team.is_picked = true;
    team.pick_content = pick_result.clone();
    team.update_time = Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local();
    app_state.context.teams.save_team(team.clone()).await.expect("save team failed");
}

async fn save_result_for_log(team_index: i32, pick_result: &String, app_state: &Data<AppState<'_>>) {
    let log = LogResponse {
        team_id: team_index,
        pick_group: pick_result.clone(),
        time: Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local(),
    };
    app_state.context.logs.save_log(log).await.expect("save log failed");
}

async fn pick_hero(app_state: &Data<AppState<'_>>) -> String {
    get_second_random_hero(get_first_random_hero(app_state).await, app_state).await
}

async fn get_first_random_hero(app_state: &Data<AppState<'_>>) -> Hero {
    let mut hero = app_state.context.heroes.get_heroes_not_is_pick().await.unwrap();
    save_hero_and_is_pick(&mut hero, app_state).await.expect("save hero failed");
    return hero;
}

async fn get_second_random_hero(exist_hero: Hero, app_state: &Data<AppState<'_>>) -> String {
    let hero = app_state.context.heroes.get_heroes_not_is_pick().await.unwrap();
    format!("[{}][{}]", exist_hero.name, hero.name)
}

async fn save_hero_and_is_pick(hero: &mut Hero, app_state: &Data<AppState<'_>>) -> Result<(), sqlx::Error> {
    hero.is_pick = true;
    app_state.context.heroes.save_hero(hero.clone()).await
}


