use std::sync::Arc;

use proto::posts::{PostRequest, PostResponse};
use tonic::{Request, Response, Status};

use super::repository::PostRepository;

pub struct PostService {
    post_service: Arc<PostRepository>,
}

#[tonic::async_trait]
pub trait PostServiceTrait {
    async fn ingest_post(
        &self,
        request: Request<PostRequest>,
    ) -> Result<Response<PostResponse>, Status>;
}

impl PostService {
    pub fn new_post_services(post_service: Arc<PostRepository>) -> Self {
        Self { post_service }
    }
}
#[tonic::async_trait]
impl PostServiceTrait for PostService {
    async fn ingest_post(
        &self,
        request: Request<PostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        println!("Получен запрос IngestPost: {:?}", request);
        let post_request = request.into_inner();
        if let Some(post_data) = post_request.post {
            println!(
                "Данные поста: chat_id={}, message_id={}, text='{}'",
                post_data.chat_id, post_data.message_id, post_data.text
            );

            let response = proto::posts::PostResponse { success: true };
            Ok(Response::new(response))
        } else {
            println!("Ошибка: поле 'post' отсутствует в запросе");
            Err(Status::invalid_argument(
                "Поле 'post' отсутствует в запросе",
            ))
        }
    }
}
