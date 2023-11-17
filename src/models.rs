use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub line: Option<i32>,
    pub is_pick: bool,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub id: Option<i32>,
    pub team_id: Option<i32>,
    pub pick_group: String,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct Team {
    pub id: Option<i32>,
    pub encrypt_code: String,
    pub is_picked: bool,
    pub update_time: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct LogResponse {
    pub team_id: Option<i32>,
    pub pick_group: String,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct MyResult {
    pub team_id: Option<i32>,
    pub data: String,
    pub time: DateTime<Utc>,
    pub logs: Option<Vec<LogResponse>>,
}

#[derive(Debug, Serialize)]
pub struct PostParam {
    pub encrypt_code: Option<String>,
}
