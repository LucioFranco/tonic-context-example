use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use hyper::server::conn::Http;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tonic::{Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

tokio::task_local! {
    static ADDR: SocketAddr;
}

#[tokio::main]
async fn main() {
    let http = Http::new().http2_only(true).clone();

    let mut listener = TcpListener::bind("[::1]:50051").await.unwrap();

    let greeter = MyGreeter::default();
    let svc = GreeterServer::new(greeter);

    while let Ok((conn, addr)) = listener.accept().await {
        let http = http.clone();
        let svc = svc.clone();

        tokio::spawn(ADDR.scope(addr, async move {
            http.serve_connection(conn, svc).await.unwrap();
        }));
    }
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", ADDR.get());

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}
