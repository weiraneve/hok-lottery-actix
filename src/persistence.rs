use chrono::{DateTime, Utc};
use actix_web::{web, http::StatusCode, Responder};
use derive_more::{Display, Error, From};
use mysql::{params, prelude::*};

use crate::{
    models::{Hero, Log, Team, MyResult, PostParam},
};

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    ServerError,
    MysqlError(mysql::Error),
    Unknown,
}

impl actix_web::ResponseError for PersistenceError {
    fn status_code(&self) -> StatusCode {
        match self {
            PersistenceError::ServerError => StatusCode::SERVICE_UNAVAILABLE,
            PersistenceError::MysqlError(_) | PersistenceError::Unknown => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

pub fn pick(param: PostParam, pool: &mysql::Pool) -> actix_web::Result<MyResult, PersistenceError> {
    let mut conn = pool.get_conn()?;
    Ok(MyResult {
        team_id: Option::from(1),
        data: param.encrypt_code.unwrap_or("123".to_string()),
        time: Utc::now(),
        logs: None,
    })
}

fn find_by_encrypt_code() {}
