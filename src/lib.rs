use std::sync::Arc;
use crate::dao::Database;

pub mod controller;
pub mod dao;
pub mod service;
pub mod model;

pub struct AppState<'a> {
    pub database: Arc<Database<'a>>
}
