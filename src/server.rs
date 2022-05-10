pub mod grpc;
mod http;

pub trait Service {
    fn next_id(&self) -> i64;

    fn next_ids(&self, step: i16) -> Vec<i64>;
}
