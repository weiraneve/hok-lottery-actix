use std::sync::Arc;
use crate::dao::Database;

pub mod controller;
pub mod dao;
pub mod service;
pub mod model;
pub mod creat_app;

pub struct AppState {
    pub database: Arc<Database>
}
