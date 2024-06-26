use serde::{Deserialize, Serialize};
use sqlx::{types::chrono::NaiveDateTime, FromRow};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Log {
    pub team_id: i32,
    #[serde(rename = "pickGroup")]
    pub pick_group: String,
    pub time: NaiveDateTime,
}
