use app::config::Config;
use proto::posts::{Post, PostRequest, post_ingest_service_client::PostIngestServiceClient};
mod app;
use chrono::Utc;
use fake::{
    Fake,
    faker::{lorem::en::Sentence, number::en::Digit},
};
#[cfg(not(target_os = "windows"))]
use jemallocator::Jemalloc as GlobalAlloc;
use prost_types::Timestamp;

#[cfg(target_os = "windows")]
use mimalloc::MiMalloc as GlobalAlloc;
use tonic::Request;

#[global_allocator]
static GLOBAL: GlobalAlloc = GlobalAlloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new().await;
    let clickhouse_config = config
        .get_grpc_config()
        .ok_or_else(|| "ClickHouse configuration not found")?;
    let addr = format!(
        "http://{}:{}",
        clickhouse_config.host, clickhouse_config.port
    );
    println!("Generator service starting...");

    let mut client = PostIngestServiceClient::connect(addr).await?;
    let post = Post {
        chat_id: 3324132432145312456,
        message_id: 12123,
        timestamp: Some(now_timestamp()),
        post_timestamp: Some(now_timestamp()),
        text: Sentence(3..100).fake(),
    };

    let req = PostRequest { post: Some(post) };

    let request = Request::new(req);
    let response = client.ingest_post(request);
    println!("Response: {:?}", response.await);
    Ok(())
}
fn now_timestamp() -> Timestamp {
    let now = Utc::now();
    Timestamp {
        seconds: now.timestamp(),
        nanos: now.timestamp_subsec_nanos() as i32,
    }
}
