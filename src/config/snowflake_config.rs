use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SnowflakeConfig {
    zookeeper_server: Option<String>,
    zone_id: Option<i8>,
    work_id: Option<i8>,
    timestamp_unit_ms: Option<i8>,
    start_time: Option<i64>,
    machine_id_bits: Option<i8>,
    sequence_bits: Option<i8>,
}
