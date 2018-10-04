#![feature(nll)]
extern crate futures;
extern crate log;
extern crate simplelog;
extern crate tikv_client;

use std::env;
use std::sync::Arc;

use simplelog::*;

use tikv_client::pd::*;
use tikv_client::util::security::{SecurityConfig, SecurityManager};

fn main() {
    let _ = TermLogger::init(LevelFilter::Info, Config::default());
    let security_manager = Arc::new(
        SecurityManager::new(&SecurityConfig::default())
            .unwrap_or_else(|e| panic!("failed to create security manager: {:?}", e)),
    );
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:3379".to_string());
    let addr = vec![addr.as_str()];
    let pd_client = PdRpcClient::new(&addr, Arc::clone(&security_manager))
        .unwrap_or_else(|e| panic!("failed to create rpc client: {:?}", e));

    println!("Cluster ID: {:?}", pd_client.get_cluster_id());
    println!("Store: {:?}", pd_client.get_store(1));
    println!("All Stores: {:?}", pd_client.get_all_stores());
    println!("Region: {:?}", pd_client.get_region(b"abc"));
    for _ in 0..10 {
        println!("TSO: {:?}", pd_client.get_ts());
    }
}
