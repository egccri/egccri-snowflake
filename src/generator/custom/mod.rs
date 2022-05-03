use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub(crate) static ref CONFIGS: HashMap<String, String> = get_config();
}

pub fn get_config() -> HashMap<String, String> {
    let mut configs = HashMap::<String, String>::with_capacity(3);
    configs.insert("a".to_string(), "b".to_string());
    configs
}
