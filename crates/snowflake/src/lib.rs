mod builder;
mod error;
mod snowflake;

pub use crate::builder::Builder;
pub use crate::error::BoxDynError;
pub use crate::snowflake::{CopySnowFlake, ShareSnowFlake};

pub trait IdGenerator {
    fn next_id(share_snowflake: &ShareSnowFlake) -> i64;

    fn next_ids(copy_snowflake: &CopySnowFlake, step: i16) -> Vec<i64>;
}

pub(crate) const TIMESTAMP_UNIT_MS: i16 = 1;
pub(crate) const TIME_BITS: i64 = 41;
pub(crate) const MACHINE_ID_BITS: i8 = 10;
pub(crate) const SEQUENCE_BITS: i8 = 12;

#[cfg(test)]
mod tests {
    use crate::builder::Builder;
    use crate::error::BoxDynError;

    #[test]
    fn it_works() {
        let share_snowflake = Builder::new()
            .machine_id(&machine_id)
            .start_time("2020-01-01 00:00:00 +08:00")
            .build()
            .unwrap();
        for _ in 0..99 {
            println!("id: {}", share_snowflake.next_id().unwrap());
        }
    }

    fn machine_id() -> Result<i16, BoxDynError> {
        Ok(16_i16)
    }
}
