use streaming::{streaming_client::StreamingClient, Start};

pub mod streaming {
    tonic::include_proto!("streaming");
}

#[tokio::main]
async fn main() {
    let mut client = StreamingClient::connect("http://[::1]:10000").await.unwrap();
    for n in 1..10 {
        println!("Requesting squares up to {}", n);
        let request = tonic::Request::new(Start { n });
        let mut stream = client.squares(request).await.unwrap().into_inner();
        while let Some(result) = stream.message().await.unwrap() {
            println!("RESULT={:?}", result);
        }
    }
}
