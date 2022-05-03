mod server_config;
mod snowflake_config;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    snowflake: snowflake_config::SnowflakeConfig,
    server: server_config::Server,
}
