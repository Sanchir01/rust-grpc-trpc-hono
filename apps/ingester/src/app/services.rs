use std::sync::Arc;

use crate::feature::post::{repository::PostRepository, service::PostService};

use super::repositories::Repositories;

#[derive(Clone)]
pub struct Services {
    pub post_service: Arc<PostService>,
}

impl Services {
    pub fn new_services(repositories: Arc<Repositories>) -> Self {
        Self {
            post_service: Arc::new(PostService::new_post_repos(
                repositories.post_repository.clone(),
            )),
        }
    }
}
