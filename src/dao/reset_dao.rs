use sqlx::MySqlPool;
use crate::model::hero::Hero;

pub async fn reset_team(id: i32, pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE `team` SET `pick_content`='',`is_picked`=0,`update_time`=current_time WHERE `id`=(?)")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn reset_all_teams(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE `team` SET `pick_content`='',`is_picked`=0,`update_time`=current_time WHERE `is_picked`=true")
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn reset_all_heroes(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE `hero` SET `is_pick`=false WHERE `is_pick`=true ")
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn find_team_by_id(id: i32, pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query_as::<_, Hero>("SELECT * FROM `team` h WHERE h.id = (?)")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(())
}
