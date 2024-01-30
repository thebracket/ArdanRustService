pub mod hello_world {
    tonic::include_proto!("tonic_auth");
}

use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tonic::Request;
use crate::hello_world::greeter_client::GreeterClient;
use crate::hello_world::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the client as a channel with a token and interceptor
    let channel = Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;
    let token: MetadataValue<_> = "Bearer some-secret-token".parse()?;
    let mut client = GreeterClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    // Using the client remains unchanged
    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}