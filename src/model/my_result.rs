use crate::model::log::Log;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyResult {
    pub team_id: i32,
    pub data: String,
    pub time: NaiveDateTime,
    pub logs: Vec<Log>,
}
