use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n Connecting to the gRPC Server...");
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    println!("\n Successfully connected to the gRPC Server!");

    let request = tonic::Request::new(HelloRequest {
        name: "gRPC Server".into(),
    });

    let response = client.say_hello(request).await?;

    println!("\nGRPC RESPONSE={:#?}", response);

    Ok(())
}
