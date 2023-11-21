use crate::dao::Table;
use crate::model::log_response::Log;
use crate::model::team::Team;

impl<'c> Table<'c, Team> {
    pub async fn get_team_by_encrypt_code(&self, encrypt_code: String) -> Result<Team, sqlx::Error> {
        sqlx::query_as::<_, Team>("SELECT * FROM `team` WHERE `encrypt_code` = ?")
            .bind(encrypt_code)
            .fetch_one(&*self.pool)
            .await
    }

    pub async fn get_log_by_team_id(&self, team_id: i32) -> Result<Vec<Log>, sqlx::Error> {
        sqlx::query_as::<_, Log>("SELECT * FROM `log` l WHERE l.team_id = ? ORDER BY l.time DESC")
            .bind(team_id.to_string())
            .fetch_all(&*self.pool)
            .await
    }

    pub async fn save_team(&self, team: Team) -> Result<(), sqlx::Error> {
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
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    pub async fn reset_team(&self, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE `team` SET `pick_content`='',`is_picked`=0,`update_time`=current_time WHERE `id`=(?)")
            .bind(id)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    pub async fn reset_all_teams(&self) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE `team` SET `pick_content`='',`is_picked`=0,`update_time`=current_time WHERE `is_picked`=true")
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    pub async fn find_team_by_id(&self, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query_as::<_, Team>("SELECT * FROM `team` h WHERE h.id = (?)")
            .bind(id)
            .fetch_one(&*self.pool)
            .await?;
        Ok(())
    }
}
