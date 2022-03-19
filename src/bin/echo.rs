use tonic::{transport::Server, Request, Response, Status};

//
// After compiling, the proto implement Rust file will located in:
//
// `target/debug/build/grpc-demo-{random-id}/out/echo.rs`/
//
// It processes by the `prost`, for more detailed info, plz visit:
//
// https://crates.io/crates/prost
//
pub mod echo {
    //
    // `echo` is the package name in your proto file!!!
    //
    // After that, this module includes:
    //
    //  `Echo`: Generated trait containing gRPC methods that should be
    //          implemented for use with EchoServer.
    //
    //  `EchoServer`: gPRC server that implements the `Echo` trait.
    //
    //  `EchoRequest` and `EchoResponse`: Messages in proto file.
    //
    tonic::include_proto!("echo");
}

use echo::{
    echo_server::{Echo, EchoServer},
    EchoRequest, EchoResponse,
};

//
// gRPC service that implements the `Echo` async trait
//
#[derive(Debug, Default)]
struct EchoService {}

#[tonic::async_trait]
impl Echo for EchoService {
    async fn say_something(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoResponse>, Status> {
        println!("Receive request: {:#?}", &request);

        // For getting back the generic type instance from `Request<EchoRequest>`,
        // use `Request.into_inner()`
        let echo_content = request.into_inner().content;

        let reply = EchoResponse {
            content: format!("[ echo from server ] - Your echo message is '{}'", echo_content),
        };

        Ok(Response::new(reply))
    }
}

//
// How to test:
//
// grpcurl -plaintext \
//      -import-path ./proto \
//      -proto echo/echo.proto \
//      -d '{"content": "Hi, just a test:)"}' \
//      127.0.0.1:6000 \
//      echo.Echo/saySomething
//
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // gPRC listent address
    let address = "127.0.0.1:6000".parse()?;

    // Create `EchoService` instance
    let echo_service = EchoService::default();

    println!("\n [ gRPC server with 'EchoService' is listening on '{}' ]\n", &address);

    // Configure gRPC server and add service to it
    Server::builder()
        .add_service(EchoServer::new(echo_service))
        .serve(address)
        .await?;

    Ok(())
}
