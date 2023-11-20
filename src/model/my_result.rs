use serde::Serialize;
use sqlx::types::chrono::NaiveDateTime;
use crate::model::log_response::LogResponse;

#[derive(Debug, Serialize)]
pub struct MyResult {
    pub team_id: i32,
    pub data: String,
    pub time: NaiveDateTime,
    pub logs: Vec<LogResponse>,
}
