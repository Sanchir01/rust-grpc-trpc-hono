use clickhouse::Client;
use std::sync::Arc;

pub struct PostRepository {
    primary_db: Client,
}

impl PostRepository {
    pub fn new_post_repository(primary_db: Client) -> Self {
        Self { primary_db }
    }
}
