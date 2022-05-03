use std::error::Error;
use thiserror::Error;

pub type BoxDynError = Box<dyn Error + 'static + Send + Sync>;

/// The error type for this crate.
#[derive(Error, Debug)]
pub enum SnowflakeError {
    #[error("start_time `{0}` is ahead of current time")]
    StartTimeAheadOfCurrentTime(String),
    #[error("machine_id returned an error: {0}")]
    MachineIdFailed(#[source] BoxDynError),
    #[error("over the time limit")]
    OverTimeLimit,
    #[error("could not find any private ipv4 address")]
    NoPrivateIPv4,
    #[error("mutex is poisoned (i.e. a panic happened while it was locked)")]
    MutexPoisoned,
}
