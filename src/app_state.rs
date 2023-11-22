use std::sync::Arc;

use sqlx::MySqlPool;

use crate::repository::hero::HeroRepositoryImpl;
use crate::repository::log::LogRepositoryImpl;
use crate::repository::team::TeamRepositoryImpl;
use crate::service::pick::PickServiceImpl;
use crate::service::reset::ResetServiceImpl;
use crate::service::Service;

pub struct AppState {
    pub service: Arc<Service>,
}

impl AppState {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        let pick_service = Arc::new(PickServiceImpl::new(
            Arc::new(HeroRepositoryImpl::new(pool.clone())),
            Arc::new(TeamRepositoryImpl::new(pool.clone())),
            Arc::new(LogRepositoryImpl::new(pool.clone())),
        ));

        let reset_service = Arc::new(ResetServiceImpl::new(
            Arc::new(HeroRepositoryImpl::new(pool.clone())),
            Arc::new(TeamRepositoryImpl::new(pool.clone())),
        ));

        AppState {
            service: Arc::new(Service {
                pick: pick_service,
                reset: reset_service,
            }),
        }
    }
}
