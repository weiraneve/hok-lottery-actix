use std::sync::Arc;

use crate::service::pick::PickService;
use crate::service::reset::ResetService;

pub mod pick;
pub mod reset;

pub struct Service {
    pub pick: Arc<dyn PickService>,
    pub reset: Arc<dyn ResetService>,
}
