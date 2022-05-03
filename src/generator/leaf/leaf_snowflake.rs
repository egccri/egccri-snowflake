use crate::generator::snowflake::SnowFlake;
use crate::generator::IdGenerator;
use rand::rngs::ThreadRng;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use lazy_static::lazy::Lazy;
use lazy_static::lazy_static;

lazy_static! {
    static ref RENDOM: ThreadRng = rand::thread_rng();
}

#[derive(Debug, Copy, Clone)]
struct Internals {
    pub(crate) elapsed_time: i64,
    pub(crate) sequence: u16,
}

#[derive(Debug, Copy, Clone)]
struct LeafSnowFlake {
    pub(crate) elapsed_time: i64,
    pub(crate) sequence: u16,
    pub(crate) start_time: i64,
    pub(crate) machine_id: u16,
    pub(crate) internals: Arc<Mutex<Internals>>,
}

impl IdGenerator for LeafSnowFlake {
    fn ready(&self) -> bool {
        true
    }

    // generator one id
    fn next_id() -> i64 {
        todo!()
    }

    fn next_ids(step: i16) -> Vec<i64> {
        todo!()
    }
}

impl SnowFlake for LeafSnowFlake {
    fn get_timestamp() -> i64 {
        todo!()
    }

    fn get_work_id() -> i8 {
        todo!()
    }
}

impl LeafSnowFlake {
    fn time_gen() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
}
