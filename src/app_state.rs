use std::sync::Arc;
use crate::dao::Database;
use crate::service::pick::PickService;

pub struct AppState {
    pub database: Arc<Database>,
    pub pick_service: Arc<dyn PickService>,
}
