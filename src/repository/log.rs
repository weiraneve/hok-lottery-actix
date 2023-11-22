use std::sync::Arc;

use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::model::log_response::Log;

#[async_trait]
pub trait LogRepository: Send + Sync {
    async fn save_log(&self, log: Log) -> Result<(), sqlx::Error>;
}

pub struct LogRepositoryImpl {
    pub pool: Arc<MySqlPool>,
}

impl LogRepositoryImpl {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        LogRepositoryImpl { pool }
    }
}

#[async_trait]
impl LogRepository for LogRepositoryImpl {
    async fn save_log(&self, log: Log) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO `log` (`team_id`, `pick_group`, `time`) VALUES (?, ?, ?)")
            .bind(log.team_id)
            .bind(log.pick_group)
            .bind(log.time)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }
}
