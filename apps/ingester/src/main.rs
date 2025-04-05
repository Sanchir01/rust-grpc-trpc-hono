use tonic::{transport::Server, Request, Response, Status};
use proto::posts::{

    post_ingest_service_server::{PostIngestService, PostIngestServiceServer}, 
    PostRequest, 
    PostResponse, 
    Post, 
};

#[derive(Default)]
pub struct MyIngesterService {} 

#[tonic::async_trait]
impl PostIngestService for MyIngesterService {
    async fn ingest_post(
        &self,
        request: Request<PostRequest>, 
    ) -> Result<Response<PostResponse>, Status> { 
        println!("Получен запрос IngestPost: {:?}", request);

     
        let post_request = request.into_inner();
       
        if let Some(post_data) = post_request.post {
             println!("Данные поста: chat_id={}, message_id={}, text='{}'",
                 post_data.chat_id,
                 post_data.message_id,
                 post_data.text);

             
             let response = proto::posts::PostResponse { success: true };
             Ok(Response::new(response))
        } else {
            println!("Ошибка: поле 'post' отсутствует в запросе");
            Err(Status::invalid_argument("Поле 'post' отсутствует в запросе"))
            
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let ingester_service = MyIngesterService::default(); // Используем новое имя

    println!("Сервер Ingester запущен на {}", addr);

    Server::builder()
        .add_service(PostIngestServiceServer::new(ingester_service))
        .serve(addr)
        .await?;

    Ok(())
}