use log::info;
use tonic::{transport::Server, Request, Response, Status};

use montinger::montinger_server::{Montinger, MontingerServer};
use montinger::{RegisterRequest, RegisterResponse};

use crate::config;

pub mod montinger {
    tonic::include_proto!("montinger");
}

#[derive(Debug, Default)]
pub struct MontingerService {}

#[tonic::async_trait]
impl Montinger for MontingerService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = RegisterResponse {
            token: format!("Hello, {}!", request.get_ref().secret),
        };

        Ok(Response::new(response))
    }
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let grpc_port = config::get_env_string("GRPC_PORT").expect("GRPC_PORT is missing.");

    let addr = format!("[::1]:{}", grpc_port).parse()?;

    info!("Starting gRPC server on [::1]:{}...", grpc_port);

    let greeter = MontingerService::default();

    Server::builder()
        .add_service(MontingerServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
