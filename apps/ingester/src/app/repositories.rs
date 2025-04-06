use std::sync::Arc;

use clickhouse::Client;

use crate::feature::post::repository::PostRepository;

#[derive(Clone)]
pub struct Repositories {
    pub post_repository: Arc<PostRepository>,
}

impl Repositories {
    pub fn new_repositories(db: Client) -> Self {
        Self {
            post_repository: Arc::new(PostRepository::new_post_repository(db.clone())),
        }
    }
}
