use log::info;
use serde::{Deserialize, Serialize};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use zookeeper::{Acl, CreateMode, WatchedEvent, Watcher, ZooKeeper};

struct LoggingWatcher;

impl Watcher for LoggingWatcher {
    fn handle(&self, e: WatchedEvent) {
        info!("{:?}", e)
    }
}

const PATH: &str = "/snowflake/meta-platform/forever";

pub fn init() {
    let zk = ZooKeeper::connect("127.0.0.1:2181", Duration::from_secs(15), LoggingWatcher);
    let zk = match zk {
        Ok(zk) => zk,
        Err(zk_error) => panic!("zookeeper connect error, {}", zk_error),
    };

    zk.add_listener(|zk_state| println!("New ZkState is {:?}", zk_state));

    let data = ReportData::new("127.0.0.1".to_string(), 8080);

    zk.create(
        "/snowflake",
        vec![],
        Acl::open_unsafe().clone(),
        CreateMode::Container,
    );
    zk.create(
        "/snowflake/meta-platform",
        vec![],
        Acl::open_unsafe().clone(),
        CreateMode::Container,
    );
    zk.create(
        "/snowflake/meta-platform/forever",
        vec![],
        Acl::open_unsafe().clone(),
        CreateMode::Container,
    );

    let path = zk.create(
        (PATH.to_string() + "127.0.0.1:8080-").as_str(),
        data.get_vec_u8(),
        Acl::open_unsafe().clone(),
        CreateMode::PersistentSequential,
    );
    println!("{:?}", path);

    sleep(Duration::from_secs(10));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportData {
    ip: String,
    port: i32,
    timestamp: i64,
}

impl ReportData {
    pub fn new(ip: String, port: i32) -> Self {
        ReportData {
            ip,
            port,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
        }
    }

    pub fn get_vec_u8(&self) -> Vec<u8> {
        println!("{:?}", self);
        serde_json::to_vec(self).unwrap()
    }
}
