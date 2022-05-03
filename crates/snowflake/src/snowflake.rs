use crate::error::SnowflakeError;
use crate::{MACHINE_ID_BITS, SEQUENCE_BITS, TIMESTAMP_UNIT_MS, TIME_BITS};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub(crate) struct Internals {
    pub(crate) elapsed_time: i64,
    pub(crate) sequence: i32,
}

#[derive(Debug)]
pub(crate) struct Snowflake {
    pub(crate) start_time: i64,
    pub(crate) machine_id: i16,
    pub(crate) internals: Mutex<Internals>,
}

/// used by next_id
#[derive(Debug, Clone)]
pub struct ShareSnowFlake(Arc<Snowflake>);

/// used by next_ids
pub struct CopySnowFlake {
    pub(crate) elapsed_time: i64,
    pub(crate) sequence: i32,
    share_snowflake: ShareSnowFlake,
}

impl ShareSnowFlake {
    pub(crate) fn new(snowflake: Snowflake) -> Self {
        Self(Arc::new(snowflake))
    }

    pub fn next_id(&self) -> Result<i64, SnowflakeError> {
        let mask_sequence = (1 << SEQUENCE_BITS) - 1;

        let mut internals = self
            .0
            .internals
            .lock()
            .map_err(|_| SnowflakeError::MutexPoisoned)?;

        // 当前时间 - 开始时间 = epouch_time
        let current = time_gen() - (self.0.start_time);
        if internals.elapsed_time < current {
            internals.elapsed_time = current;
            internals.sequence = 0;
        } else {
            // self.elapsed_time >= current
            internals.sequence = (internals.sequence + 1) & mask_sequence;
            if internals.sequence == 0 {
                internals.elapsed_time += 1;
                let overtime = internals.elapsed_time - current;
                thread::sleep(sleep_time(overtime));
            }
        }

        if internals.elapsed_time >= 1 << TIME_BITS {
            return Err(SnowflakeError::OverTimeLimit);
        }

        Ok(
            (internals.elapsed_time as i64) << (SEQUENCE_BITS + MACHINE_ID_BITS)
                | (self.0.machine_id as i64) << SEQUENCE_BITS
                | (internals.sequence as i64),
        )
    }
}

impl CopySnowFlake {
    fn next_ids(&self, step: i8) {}
}

fn time_gen() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

fn sleep_time(overtime: i64) -> Duration {
    Duration::from_millis(overtime as u64)
}
