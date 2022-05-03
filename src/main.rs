mod config;
mod generator;
mod server;

use clap::Parser;
use tonic::transport::Server;
use server::grpc::SnowflakeGrpcService;
use server::grpc::snowflake_grpc::snowflake_service_server::SnowflakeServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // generator::snowflake::snowflake_zookeeper_holder::init();

    let addr = "[::1]:50051".parse()?;
    let snowflake_service = SnowflakeGrpcService::default();

    Server::builder()
        .add_service(SnowflakeServiceServer::new(snowflake_service))
        .serve(addr)
        .await?;

    Ok(())


    // let args = Args::parse();
    // println!("{:?}", args);
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    config: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}
