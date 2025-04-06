use std::sync::Arc;

use crate::feature::post::handler::MyIngesterHandlers;

use super::services::Services;

#[derive(Clone)]
pub struct Handlers {
    pub post_handler: Arc<MyIngesterHandlers>,
}

impl Handlers {
    pub fn new_handlers(services: Arc<Services>) -> Self {
        Self {
            post_handler: Arc::new(MyIngesterHandlers::new_handlers(
                services.post_service.clone(),
            )),
        }
    }
}
