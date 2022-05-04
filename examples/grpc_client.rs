use snowflake_grpc::snowflake_service_client::SnowflakeServiceClient;
use snowflake_grpc::IdRequest;

pub mod snowflake_grpc {
    include!(concat!(env!("OUT_DIR"), "/snowflake_grpc.rs")); // The string specified here must match the proto package name
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SnowflakeServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(IdRequest {});

    let response = client.get_id(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}