use crate::app::config::Config;
use crate::feature::post::handler::MyIngesterHandlers;
use proto::posts::{
    PostRequest, PostResponse,
    post_ingest_service_server::{PostIngestService, PostIngestServiceServer},
};
use std::{error::Error as StdError, net::SocketAddr, sync::Arc};
use tonic::{Request, Response, Status, transport::Server};
struct ArcWrapper(Arc<MyIngesterHandlers>);

#[tonic::async_trait]
impl PostIngestService for ArcWrapper {
    async fn ingest_post(
        &self,
        request: Request<PostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        self.0.ingest_post(request).await
    }
}
pub async fn init_grpc_server(
    config: Config,
    ingester_handler: Arc<MyIngesterHandlers>,
) -> Result<(), Box<dyn StdError>> {
    let rpc_config = config
        .get_grpc_config()
        .expect("gRPC server configuration is missing in the config file");

    let addr = format!("{}:{}", rpc_config.host, rpc_config.port).parse::<SocketAddr>()?;
    println!("gRPC Server listening on: {}", addr);

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::posts::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    Server::builder()
        .add_service(service)
        .add_service(PostIngestServiceServer::new(ArcWrapper(ingester_handler)))
        .serve(addr)
        .await?;

    Ok(())
}
