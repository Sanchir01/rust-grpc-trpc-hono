use super::entity::PostStruct;
use chrono::{DateTime, Utc};
use clickhouse::Client;

pub struct PostRepository {
    primary_db: Client,
}
#[tonic::async_trait]
pub trait PostRepositoryTrait {
    async fn create_post(&self, post: PostStruct) -> Result<(), clickhouse::error::Error>;
}

impl PostRepository {
    pub fn new_post_repository(primary_db: Client) -> Self {
        Self { primary_db }
    }
}
#[tonic::async_trait]
impl PostRepositoryTrait for PostRepository {
    async fn create_post(&self, post: PostStruct) -> Result<(), clickhouse::error::Error> {
        println!("repos post{:?}", post);
        let sql = r#"
             INSERT INTO posts (chat_id, message_id, timestamp, post_timestamp, text)
             VALUES (?, ?, ?, ?, ?)
         "#;
        self.primary_db
            .query(sql)
            .bind(post.chat_id)
            .bind(post.message_id)
            .bind(format_without_z(&post.timestamp))
            .bind(format_without_z(&post.post_timestamp))
            .bind(post.text)
            .execute()
            .await?;
        Ok(())
    }
}
fn format_without_z(dt: &DateTime<Utc>) -> String {
    dt.format("%Y-%m-%dT%H:%M:%S.%3f").to_string() // Без 'Z' на конце
}
