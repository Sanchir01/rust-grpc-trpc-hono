use crate::feature::post::{entity::PostStruct, repository::PostRepositoryTrait};
use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use proto::posts::{PostRequest, PostResponse};
use std::sync::Arc;
use tonic::{Request, Response, Status};

use super::repository::PostRepository;

pub struct PostService {
    post_repo: Arc<PostRepository>,
}

#[tonic::async_trait]
pub trait PostServiceTrait {
    async fn ingest_post(
        &self,
        request: Request<PostRequest>,
    ) -> Result<Response<PostResponse>, Status>;
}

impl PostService {
    pub fn new_post_repos(post_repo: Arc<PostRepository>) -> Self {
        Self { post_repo }
    }
}
#[tonic::async_trait]
impl PostServiceTrait for PostService {
    async fn ingest_post(
        &self,
        request: Request<PostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        let post_request = request.into_inner();

        let post_data = match post_request.post {
            Some(data) => data,
            None => {
                println!("Ошибка: поле 'post' отсутствует в запросе");
                return Err(Status::invalid_argument(
                    "Поле 'post' отсутствует в запросе",
                ));
            }
        };

        let post_struct = PostStruct::from(post_data);

        let result = self.post_repo.create_post(post_struct).await;

        if let Err(e) = result {
            return Err(Status::internal(format!(
                "Ошибка при создании поста: {}",
                e
            )));
        }

        let response = proto::posts::PostResponse { success: true };
        Ok(Response::new(response))
    }
}
