use serde::Serialize;
use sqlx::{FromRow, types::chrono::NaiveDateTime};

#[derive(Debug, Serialize, FromRow, Clone)]
pub struct LogResponse {
    pub team_id: i32,
    #[serde(rename = "pickGroup")]
    pub pick_group: String,
    pub time: NaiveDateTime,
}
