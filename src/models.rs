use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::chrono::NaiveDateTime};

#[derive(Debug, Deserialize)]
pub struct PostParam {
    #[serde(rename = "encryptCode")]
    pub encrypt_code: String,
}

#[derive(Debug, Deserialize)]
pub struct Hero {
    pub id: i32,
    pub name: String,
    pub line: i32,
    pub is_pick: bool,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub id: i32,
    pub team_id: i32,
    pub pick_group: String,
    pub time: NaiveDateTime,
}

#[derive(Debug, FromRow)]
pub struct Team {
    pub id: i32,
    pub encrypt_code: String,
    pub pick_content: String,
    pub is_picked: bool,
    pub update_time: NaiveDateTime,
}

#[derive(Debug, Serialize, FromRow)]
pub struct LogResponse {
    pub team_id: i32,
    #[serde(rename = "pickGroup")]
    pub pick_group: String,
    pub time: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct MyResult {
    pub team_id: i32,
    pub data: String,
    pub time: NaiveDateTime,
    pub logs: Vec<LogResponse>,
}
