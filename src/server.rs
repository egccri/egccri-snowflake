pub mod grpc;
mod http;

pub trait Service {
    fn next_id() -> i64;

    fn next_ids(step: i16) -> Vec<i64>;
}
