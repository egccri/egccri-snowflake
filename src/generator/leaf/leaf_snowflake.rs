use crate::generator::leaf::snowflake_zookeeper_holder::ZookeeperHolder;
use crate::server::Service;
use snowflake::{BoxDynError, Builder, IdGenerator};

pub struct LeafSnowflake {
    start_time: String,
    work_id: Option<i32>,
}

impl LeafSnowflake {
    pub fn new() -> Self {
        let start_time = "2020-01-01 00:00:00 +08:00";
        let ip = "127.0.0.1";
        let port = 50052_i32;
        let zookeeper_address = "127.0.0.1:2181";
        let mut zookeeper_holder = ZookeeperHolder::new(zookeeper_address, ip, port);
        zookeeper_holder.init();
        let work_id = zookeeper_holder.get_work_id();
        LeafSnowflake {
            start_time: start_time.to_string(),
            work_id,
        }
    }

    fn machine_id(&self) -> Option<i32> {
        self.work_id
    }
}

impl Service for LeafSnowflake {
    fn next_id(&self) -> i64 {
        let share_snowflake = Builder::new()
            .machine_id(self.work_id)
            .start_time("2020-01-01 00:00:00 +08:00")
            .build()
            .unwrap();
        share_snowflake.next_id().unwrap()
    }

    fn next_ids(&self, step: i16) -> Vec<i64> {
        todo!()
    }
}

impl IdGenerator for LeafSnowflake {
    fn next_id(share_snowflake: &snowflake::ShareSnowFlake) -> i64 {
        todo!()
    }

    fn next_ids(copy_snowflake: &snowflake::CopySnowFlake, step: i16) -> Vec<i64> {
        todo!()
    }
}
