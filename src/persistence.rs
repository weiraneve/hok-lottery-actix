use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};
use mysql::{params, prelude::*};

use crate::{
    models::{Hero, Log, Team}
};

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    MysqlError(mysql::Error),
    Unknown,
}

impl actix_web::ResponseError for PersistenceError {
    fn status_code(&self) -> StatusCode {
        match self {
            PersistenceError::MysqlError(_) | PersistenceError::Unknown => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}
