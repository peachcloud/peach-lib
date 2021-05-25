//! Make HTTP requests to the `peach-dyndns-server` API which runs on the peach-vps.
//! if the requests are successful, configurations are saved locally on peachcloud appropriately
//!
//! the domain for dyndns updates is stored in /var/lib/peachcloud/config.yml
//! the tsig key for authenticating the updates is stored in /var/lib/peachcloud/peach-dyndns/tsig.key
use log::{debug, info};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::str::ParseBoolError;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
pub mod config;
pub mod error;
use crate::config::{set_peach_dyndns_config, PeachDynDnsConfig};
use crate::error::PeachError;
use jsonrpc_client_core::{expand_params, jsonrpc_client};
use jsonrpc_client_http::HttpTransport;
use serde_json::Value;

pub const PEACH_DYNDNS_URL: &str = "http://dynserver.dyn.peachcloud.org";
pub const TSIG_KEY_PATH: &str = "/var/lib/peachcloud/peach-dyndns/tsig.key";
pub const PEACH_DYNDNS_CONFIG_PATH: &str = "/var/lib/peachcloud/peach-dyndns";
pub const DYNDNS_LOG_PATH: &str = "/var/lib/peachcloud/peach-dyndns/latest_result.log";

// helper function which saves dyndns TSIG key returned by peach-dyndns-server to /var/lib/peachcloud/peach-dyndns/tsig.key
pub fn save_dyndns_key(key: &str) {
    // create directory if it doesn't exist
    fs::create_dir_all(PEACH_DYNDNS_CONFIG_PATH)
        .expect(&format!("Failed to create: {}", PEACH_DYNDNS_CONFIG_PATH));
    // write key text
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(TSIG_KEY_PATH)
        .expect(&format!("failed to open {}", TSIG_KEY_PATH));
    writeln!(file, "{}", key).expect(&format!("Couldn't write to file: {}", TSIG_KEY_PATH));
}

/// Makes a post request to register a new domain with peach-dyns-server
/// if the post is successful, the domain is registered with peach-dyndns-server
/// a unique TSIG key is returned and saved to disk,
/// and peachcloud is configured to start updating the IP of this domain using nsupdate
pub fn register_domain(domain: &str) -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for dyndns client.");
    let transport = HttpTransport::new().standalone()?;
    let http_server = PEACH_DYNDNS_URL;
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachDynDnsClient::new(transport_handle);

    info!("Performing register_domain call to peach-dyndns-server");
    let res = client.register_domain(&domain).call();
    match res {
        Ok(key) => {
            save_dyndns_key(&key);
            let new_peach_dyn_dns_config = PeachDynDnsConfig {
                domain: domain.to_string(),
                dns_server_address: PEACH_DYNDNS_URL.to_string(),
                tsig_key_path: TSIG_KEY_PATH.to_string(),
            };
            set_peach_dyndns_config(new_peach_dyn_dns_config);
            let response = "success".to_string();
            Ok(response)
        }
        Err(err) => Err(PeachError::JsonRpcCore(err)),
    }
}

/// Makes a post request to check if a domain is available
pub fn is_domain_available(domain: &str) -> std::result::Result<bool, PeachError> {
    debug!("Creating HTTP transport for dyndns client.");
    let transport = HttpTransport::new().standalone()?;
    let http_server = PEACH_DYNDNS_URL;
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachDynDnsClient::new(transport_handle);

    info!("Performing register_domain call to peach-dyndns-server");
    let res = client.is_domain_available(&domain).call();
    info!("res: {:?}", res);
    match res {
        Ok(result_str) => {
            let result : Result<bool, ParseBoolError> = FromStr::from_str(&result_str);
            match result {
                Ok(result_bool) => Ok(result_bool),
                Err(err) => Err(PeachError::ParseBoolError(err)),
            }
        }
        Err(err) => Err(PeachError::JsonRpcCore(err)),
    }
}



jsonrpc_client!(pub struct PeachDynDnsClient {
    pub fn register_domain(&mut self, domain: &str) -> RpcRequest<String>;
    pub fn is_domain_available(&mut self, domain: &str) -> RpcRequest<String>;
});

// main fn for testing
fn main() -> () {
    // initalize the logger
    env_logger::init();

    let test_domain = "newquartet10.dyn.peachcloud.org";

    let result = register_domain(test_domain);

    match result {
        Ok(key) => println!("returned key: {:?}", key),
        Err(err) => println!("err: {:?}", err),
    }

    let test_domain = "newquartet10.dyn.peachcloud.org";
    let result = is_domain_available(test_domain);
    match result {
        Ok(r) => println!("is available: {:?}", r),
        Err(err) => println!("err: {:?}", err)
    }

    ()
}
