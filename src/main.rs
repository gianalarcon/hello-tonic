use tonic::{transport::Server, Request, Response, Status};
use tonic::transport::Channel;
use hello::greeter_server::{Greeter, GreeterServer};
use hello::greeter_client::GreeterClient;
use hello::{HelloReply, HelloRequest};

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let x = request.into_inner();
        let reply = HelloReply {
            message: format!("Hi {}!, your gender:{}, your age:{}, your company:{}, your department:{}, your id:{}, your location:{}",
             x.name, x.gender, x.age, x.company, x.department, x.id, x.location).into(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_say_hello() {
    // Start your gRPC server in the background
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();
    let server = tokio::spawn(async move {
        Server::builder()
            .add_service(GreeterServer::new(greeter))
            .serve(addr)
            .await
            .unwrap();
    });

    // Wait for the server to start
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // Create a channel to connect to the server
    let channel = Channel::from_static("http://[::1]:50051")
        .connect()
        .await
        .unwrap();

    // Create a client using the channel
    let mut client = GreeterClient::new(channel);

    // Create a request
    let request = hello::HelloRequest {
        name: "Tonic".into(),
        gender: "Male".into(),
        age: "25".into(),
        company: "Axum".into(),
        department: "Engineering".into(),
        id: "123".into(),
        location: "Lagos".into()};
    
    // Call the grpc service method
    let response = client.say_hello(Request::new(request)).await.unwrap();

    // Assert that the response is what you expect
    assert_eq!(response.into_inner().message, "Hi Tonic!, your gender:Male, your age:25, your company:Axum, your department:Engineering, your id:123, your location:Lagos");

    // Shutdown the server
    server.abort();
}
