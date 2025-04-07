

#[cfg(not(target_os = "windows"))]
use jemallocator::Jemalloc as GlobalAlloc;
use proto::posts::post_ingest_service_client::PostIngestServiceClient;

#[cfg(target_os = "windows")]
use mimalloc::MiMalloc as GlobalAlloc;

#[global_allocator]
static GLOBAL: GlobalAlloc = GlobalAlloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://localhost:50051";
    let config = Config::new().await;
    println!("Generator service starting...");

    let mut client = PostIngestServiceClient::connect(url).await?;
    Ok(())
}
