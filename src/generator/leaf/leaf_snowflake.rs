use snowflake::IdGenerator;

pub struct LeafSnowflake {}

impl IdGenerator for LeafSnowflake {
    fn next_id(share_snowflake: &snowflake::ShareSnowFlake) -> i64 {
        todo!()
    }

    fn next_ids(copy_snowflake: &snowflake::CopySnowFlake, step: i16) -> Vec<i64> {
        todo!()
    }
}
