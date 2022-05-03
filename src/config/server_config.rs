use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Server {
    grpc: Grpc,
    http: Http,
}

#[derive(Debug, Deserialize)]
pub struct Grpc {}

#[derive(Debug, Deserialize)]
pub struct Http {}
