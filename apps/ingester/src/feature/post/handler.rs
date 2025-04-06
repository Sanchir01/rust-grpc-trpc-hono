use crate::feature::post::service::{PostService, PostServiceTrait};
use proto::posts::{
    Post, PostRequest, PostResponse, post_ingest_service_server::PostIngestService,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct MyIngesterHandlers {
    pub post_service: Arc<PostService>,
}

impl MyIngesterHandlers {
    pub fn new_handlers(post_service: Arc<PostService>) -> Self {
        Self { post_service }
    }
}

#[tonic::async_trait]
impl PostIngestService for MyIngesterHandlers {
    async fn ingest_post(
        &self,
        request: Request<PostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        self.post_service.ingest_post(request).await
    }
}
