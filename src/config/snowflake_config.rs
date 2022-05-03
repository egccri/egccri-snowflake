#[derive(Debug)]
pub struct SnowFlakeConfig {
    data_center_id: i8,
    worker_id: Option<i8>,
    zookeeper_server: Option<String>,
}
