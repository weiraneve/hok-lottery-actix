use crate::dao::Table;
use crate::model::log_response::Log;

impl<'c> Table<'c, Log> {
    pub async fn save_log(&self, log: Log) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO `log` (`team_id`, `pick_group`, `time`) VALUES (?, ?, ?)")
            .bind(log.team_id)
            .bind(log.pick_group)
            .bind(log.time)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }
}
