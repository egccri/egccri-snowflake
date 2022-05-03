use std::error::Error;
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::error::{BoxDynError, SnowFlakeError};
use crate::snowflake::SnowFlake;

pub struct Builder<'a> {
    start_time: Option<SystemTime>,
    machine_id: Option<&'a dyn Fn() -> Result<u16, BoxDynError>>,
}

impl <'a> Builder<'a> {
    pub fn new() -> Self {
        Self {
            start_time: None,
            machine_id: None,
        }
    }

    pub fn start_time(mut self, start_time: SystemTime) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn machine_id(mut self, machine_id: &'a dyn Fn() -> Result<i16, BoxDynError>) -> Self {
        self.machine_id = Some(machine_id);
        self
    }

    pub fn build(self) -> Result<SnowFlake, SnowFlakeError> {

       let start_time = if let Some(start_time) = self.start_time {
           let ms = SystemTime::now().duration_since(start_time).unwrap().as_millis();
           if ms > 0 {
               return Err(SnowFlakeError::StartTimeAheadOfCurrentTime(start_time));
           }
           ms as i64
       } else {
           SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64
       };

        let machine_id = if let Some(machine_id) = self.machine_id {
            match machine_id() {
                Ok(machine_id) => machine_id,
                Err(e) => return Err(Error::MachineIdFailed(e)),
            }
        } else {

        };

        Ok(SnowFlake {
            start_time,
            machine_id: 0,
            internals: Mutex::new(Internals {})
        })
    }
}