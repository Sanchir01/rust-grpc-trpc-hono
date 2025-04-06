use clickhouse::Row;
use serde::Deserialize;

#[derive(Debug, Row, Deserialize)]
struct Post {
    pub chat_id: i64,
    pub message_id: u32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub post_timestamp: chrono::DateTime<chrono::Utc>,
    pub text: String,
}
