use tonic::{transport::Server, Request, Response, Status};
use tokio_stream::wrappers::ReceiverStream;

pub mod streaming {
    tonic::include_proto!("streaming");
}
use streaming::streaming_server::{ Streaming, StreamingServer};
use streaming::{Start, Square};

#[derive(Debug, Default)]
pub struct StreamingService {}

#[tonic::async_trait]
impl Streaming for StreamingService {
    type SquaresStream = ReceiverStream<Result<Square, Status>>;

    async fn squares(&self, request: Request<Start>) -> Result<Response<Self::SquaresStream>, Status> {
        println!("Got a request: {:?}", request);
        let (tx, rx) = tokio::sync::mpsc::channel(4);

        tokio::spawn(async move {
            for i in 0..request.into_inner().n {
                let square = Square {
                    n: i * i,
                };
                tx.send(Ok(square)).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();
    println!("Square Server listening on: {}", addr);

    let streamer = StreamingService {};
    let svc = StreamingServer::new(streamer);
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}