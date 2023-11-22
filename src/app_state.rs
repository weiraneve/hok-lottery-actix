use std::sync::Arc;
use crate::dao::Database;

pub struct AppState {
    pub database: Arc<Database>
}
