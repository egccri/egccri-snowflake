use crate::error::{BoxDynError, SnowflakeError};
use crate::snowflake::Internals;
use crate::snowflake::Snowflake;
use crate::ShareSnowFlake;
use chrono::{DateTime, Utc};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Builder<'a> {
    start_time: Option<i64>,
    machine_id: Option<&'a dyn Fn() -> Result<i16, BoxDynError>>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Self {
            start_time: None,
            machine_id: None,
        }
    }

    pub fn start_time(mut self, start_time: &str) -> Self {
        let datetime = DateTime::parse_from_str(start_time, "%Y-%m-%d %H:%M:%S %z");
        self.start_time = Some(datetime.unwrap().timestamp_millis());
        self
    }

    pub fn machine_id(mut self, machine_id: &'a dyn Fn() -> Result<i16, BoxDynError>) -> Self {
        self.machine_id = Some(machine_id);
        self
    }

    pub fn build(self) -> Result<ShareSnowFlake, SnowflakeError> {
        let start_time = if let Some(start_time) = self.start_time {
            let current = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64;
            if (current - start_time) < 0 {
                return Err(SnowflakeError::StartTimeAheadOfCurrentTime(
                    // DateTime::from_utc(Utc.timestamp_millis(start_time), 8).to_string(),
                    start_time.to_string(),
                ));
            }
            start_time
        } else {
            DateTime::parse_from_str("2020-01-01 00:00:00 +08:00", "%Y-%m-%d %H:%M:%S %z")
                .unwrap()
                .timestamp_millis()
        };

        let machine_id = if let Some(machine_id) = self.machine_id {
            match machine_id() {
                Ok(machine_id) => machine_id,
                Err(e) => return Err(SnowflakeError::MachineIdFailed(e)),
            }
        } else {
            0_i16
        };

        Ok(ShareSnowFlake::new(Snowflake {
            start_time,
            machine_id: machine_id,
            internals: Mutex::new(Internals {
                elapsed_time: 0,
                sequence: 0,
            }),
        }))
    }
}
