use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use zookeeper::{Acl, CreateMode, Stat, WatchedEvent, Watcher, ZkResult, ZooKeeper};

struct LoggingWatcher;

impl Watcher for LoggingWatcher {
    fn handle(&self, e: WatchedEvent) {
        info!("{:?}", e)
    }
}

const PATH: &str = "/snowflake/com.egccri.meta/forever";

#[derive(Debug)]
pub struct ZookeeperHolder {
    zk_address: String,
    snowflake_address: String,
    work_id: Option<i32>,
    node_path: Option<String>,
}

impl ZookeeperHolder {
    pub fn new(zk_address: &str, snowflake_ip: &str, snowflake_port: i32) -> Self {
        Self {
            zk_address: zk_address.to_string(),
            snowflake_address: snowflake_ip.to_string() + ":" + snowflake_port.to_string().as_str(),
            work_id: None,
            node_path: None,
        }
    }

    pub fn init(&mut self) {
        let zk = ZooKeeper::connect(
            self.zk_address.as_str(),
            Duration::from_secs(15),
            LoggingWatcher,
        );
        let zk = match zk {
            Ok(zk) => zk,
            Err(zk_error) => panic!("zookeeper connect error, {}", zk_error),
        };

        zk.add_listener(|zk_state| println!("New ZkState is {:?}", zk_state));

        self.begin(&zk);

        // thread::spawn(move || {
        //     thread::sleep(Duration::from_secs(3));
        //     let data = ReportData::new(self.snowflake_address.clone());
        //     zk.set_data((PATH.to_owned() + self.node_path.unwrap().as_str()).as_str(), data.get_vec_u8(), None);
        // });
    }

    fn begin(&mut self, zk: &ZooKeeper) {
        let exists = zk.exists(PATH, false);

        match exists {
            Ok(stat) => {
                match stat {
                    None => {
                        // 没有创建过根节点
                        create_root_path(&zk);
                        self.create_node(&zk);
                        thread::sleep(Duration::from_millis(100));
                        self.begin(zk)
                    }
                    Some(_) => {
                        // 已存在根节点，检查是子节点否第一次注册
                        let server_list = zk.get_children(PATH, false).unwrap();
                        let node_map: HashMap<&str, &str> = server_list
                            .iter()
                            .filter_map(|server| server.split_once("-"))
                            .collect();
                        if let Some(work_id) = node_map
                            .get(self.snowflake_address.as_str())
                            .map(|v| v.to_string().parse::<i32>().unwrap())
                        {
                            // 子节点已注册
                            println!("{}", work_id);
                            self.work_id = Some(work_id);
                            self.node_path = Some(
                                self.snowflake_address.clone()
                                    + node_map.get(self.snowflake_address.as_str()).unwrap(),
                            )
                        } else {
                            // 子节点未注册，注册子节点
                            self.create_node(&zk);
                            thread::sleep(Duration::from_millis(100));
                            self.begin(zk)
                        }
                    }
                }
            }
            Err(zk_error) => {
                panic!("zookeeper connect error, {}", zk_error)
            }
        }
    }

    pub fn get_work_id(&self) -> Option<i32> {
        self.work_id
    }

    fn create_node(&self, zk: &ZooKeeper) -> ZkResult<String> {
        let path = PATH.to_string() + "/" + self.snowflake_address.clone().as_str() + "-";
        let data = ReportData::new(self.snowflake_address.clone());
        zk.create(
            path.as_str(),
            data.get_vec_u8(),
            Acl::open_unsafe().clone(),
            CreateMode::PersistentSequential,
        )
    }
}

fn create_root_path(zk: &ZooKeeper) {
    zk.create(
        "/snowflake",
        vec![],
        Acl::open_unsafe().clone(),
        CreateMode::Container,
    );
    zk.create(
        "/snowflake/com.egccri.meta",
        vec![],
        Acl::open_unsafe().clone(),
        CreateMode::Container,
    );
    zk.create(
        "/snowflake/com.egccri.meta/forever",
        vec![],
        Acl::open_unsafe().clone(),
        CreateMode::Container,
    );
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportData {
    address: String,
    timestamp: i64,
}

impl ReportData {
    pub fn new(address: String) -> Self {
        ReportData {
            address,
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
