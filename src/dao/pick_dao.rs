use sqlx::MySqlPool;
use crate::model::hero::Hero;
use crate::model::log_response::LogResponse;
use crate::model::team::Team;

pub async fn get_team_by_encrypt_code(encrypt_code: String, pool: &MySqlPool) -> Result<Team, sqlx::Error> {
    sqlx::query_as::<_, Team>("SELECT * FROM `team` WHERE `encrypt_code` = ?")
        .bind(encrypt_code)
        .fetch_one(pool)
        .await
}

pub async fn get_log_by_team_id(team_id: i32, pool: &MySqlPool) -> Result<Vec<LogResponse>, sqlx::Error> {
    sqlx::query_as::<_, LogResponse>("SELECT * FROM `log` l WHERE l.team_id = ? ORDER BY l.time DESC")
        .bind(team_id.to_string())
        .fetch_all(pool)
        .await
}

pub async fn get_heroes_not_is_pick(pool: &MySqlPool) -> Result<Hero, sqlx::Error> {
    sqlx::query_as::<_, Hero>("SELECT * FROM `hero` h WHERE h.is_pick = FALSE ORDER BY RAND() LIMIT 1")
        .fetch_one(pool)
        .await
}

pub async fn save_hero(hero: Hero, pool: &MySqlPool) -> Result<(), sqlx::Error> {
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

pub async fn save_log(log: LogResponse, pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO `log` (`team_id`, `pick_group`, `time`) VALUES (?, ?, ?)")
        .bind(log.team_id)
        .bind(log.pick_group)
        .bind(log.time)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn save_team(team: Team, pool: &MySqlPool) -> Result<(), sqlx::Error> {
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
