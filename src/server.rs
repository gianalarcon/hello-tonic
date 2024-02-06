use tonic::{transport::Server, Request, Response, Status};

use hello::greeter_server::{Greeter, GreeterServer};
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
