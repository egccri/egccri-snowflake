use crate::server::Service;
use lazy_static::lazy_static;
use snowflake::{BoxDynError, Builder};
use std::collections::HashMap;

lazy_static! {
    pub(crate) static ref CONFIGS: HashMap<String, String> = get_config();
}

pub fn get_config() -> HashMap<String, String> {
    let mut configs = HashMap::<String, String>::with_capacity(3);
    configs.insert("a".to_string(), "b".to_string());
    configs
}

#[derive(Debug, Default)]
pub struct CustomSnowflake {}

impl Service for CustomSnowflake {
    fn next_id(&self) -> i64 {
        let share_snowflake = Builder::new()
            .machine_id(Some(16))
            .start_time("2020-01-01 00:00:00 +08:00")
            .build()
            .unwrap();
        share_snowflake.next_id().unwrap()
    }

    fn next_ids(&self, step: i16) -> Vec<i64> {
        todo!()
    }
}

fn machine_id() -> Result<i16, BoxDynError> {
    Ok(16_i16)
}
