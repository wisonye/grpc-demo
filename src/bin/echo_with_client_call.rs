use tonic::{transport::Server, Request, Response, Status};

//
//
//
pub mod echo {
    tonic::include_proto!("echo");
}

use echo::{
    echo_client::EchoClient,
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
    //
    //
    //
    async fn say_something(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoResponse>, Status> {
        println!("[ Server ] >>> Receive request: {request:#?}\n");

        // For getting back the generic type instance from `Request<EchoRequest>`,
        // use `Request.into_inner()`
        let echo_content = request.into_inner().content;

        let reply = EchoResponse {
            content: format!(
                "[ echo from server ] - Your echo message is '{}'",
                echo_content
            ),
        };

        Ok(Response::new(reply))
    }
}

//
//
//
async fn run_client_call() -> () {
    let mut connected_client = EchoClient::connect("http://0.0.0.0:6000").await;
    while connected_client.is_err() {
        println!(">>> client fails to connect to server, retry again......");
        connected_client = EchoClient::connect("0.0.0.0:6000").await;
    }

    println!("[ Client ] >>> Client connected to server successfully.\n");

    let request = EchoRequest {
        content: "Just a test:)".to_string(),
    };

    let response = match connected_client.unwrap().say_something(request).await {
        Ok(response) => {
            format!("{}", response.into_inner().content)
        }
        Err(error) => error.to_string(),
    };

    println!("\n[ Client ] >>> Got response: {response:?}\n");

    ()
}

//
//
//
async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    // gPRC listent address
    let address = "0.0.0.0:6000".parse()?;

    // Create `EchoService` instance
    let echo_service = EchoService::default();

    println!(
        "\n [ gRPC server with 'EchoService' is listening on '{}' ]\n",
        &address
    );

    // Configure gRPC server and add service to it
    Server::builder()
        .add_service(EchoServer::new(echo_service))
        .serve(address)
        .await?;

    Ok(())
}

//
//
//
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_, _) = tokio::join!(run_server(), run_client_call());

    Ok(())
}
