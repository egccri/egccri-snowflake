use tonic::{transport::Server, Request, Response, Status};

use snowflake_grpc::snowflake_service_server::{SnowflakeService, SnowflakeServiceServer};
use snowflake_grpc::{IdRequest, IdReply, IdsRequest, IdsReply};
use crate::generator::custom::CustomSnowflake;
use crate::server::Service;


pub mod snowflake_grpc {
    include!(concat!(env!("OUT_DIR"), "/snowflake_grpc.rs")); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct SnowflakeGrpcService {}

#[tonic::async_trait]
impl SnowflakeService for SnowflakeGrpcService {
    async fn get_id(
        &self,
        request: Request<IdRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<IdReply>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = snowflake_grpc::IdReply {
            message: CustomSnowflake::next_id(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn get_ids(
        &self,
        request: Request<IdsRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<IdsReply>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = snowflake_grpc::IdsReply {
            message: 1, // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

