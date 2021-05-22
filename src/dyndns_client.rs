//! Make HTTP requests to the `peach-dyndns-server` API which runs on the peach-vps.
//! if the requests are successful, configurations are saved locally on peachcloud appropriately
//!
//! the domain for dyndns updates is stored in /var/lib/peachcloud/config.yml
//! the tsig key for authenticating the udpates is stored in /var/lib/peachcloud/peach-dyndns/tsig.key
use std::env;
use std::fs;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use reqwest::blocking::Client;
pub mod config;
use crate::config::{set_peach_dyndns_config, PeachDynDnsConfig};
use serde_json::Value;

pub const PEACH_DYNDNS_URL : &str = "http://dynserver.dyn.peachcloud.org";
pub const TSIG_KEY_PATH : &str = "/var/lib/peachcloud/peach-dyndns/tsig.key";
pub const PEACH_DYNDNS_CONFIG_PATH : &str = "/var/lib/peachcloud/peach-dyndns";
pub const DYNDNS_LOG_PATH : &str = "/var/lib/peachcloud/peach-dyndns/latest_result.log";

// the type returned by peach-dyndns-server requests
// note that this type is defined slightly differently in peach-dyndns-server,
// where data is of type Option<rocket::contrib::JsonValue>
// and here data is of type Option<serde_json::Value>
// serializing and deserializing between these types is ok as explained here
// https://api.rocket.rs/v0.4/rocket_contrib/json/struct.JsonValue.html
#[derive(Deserialize, Debug)]
pub struct JsonResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

impl JsonResponse {
    fn success(&self) -> bool {
        return self.status == "success"
    }
}

#[derive(Debug)]
pub enum PeachDynDnsError {
    ServerError(JsonResponse),
    InvalidServerResponse(JsonResponse),
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for PeachDynDnsError {
    fn from(err: reqwest::Error) -> PeachDynDnsError {
        PeachDynDnsError::ReqwestError(err)
    }
}


// helper function which saves dyndns TSIG key returned by peach-dyndns-server to /var/lib/peachcloud/peach-dyndns/tsig.key
pub fn save_dyndns_key(key: &str) {
    // create directory if it doesn't exist
    fs::create_dir_all(PEACH_DYNDNS_CONFIG_PATH).expect(&format!("Failed to create: {}", PEACH_DYNDNS_CONFIG_PATH));
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
/// and peachcloud is configured to start updating the IP of this domain using nsupdate
pub fn register_domain(domain: &str) -> std::result::Result<(), PeachDynDnsError> {
    // This will POST a body of `{"lang":"rust","body":"json"}`
    let mut map = HashMap::new();
    map.insert("domain", domain);

    let client = Client::new();
    let api_url = PEACH_DYNDNS_URL.to_owned() + "/domain/register";
    let res = client.post(api_url)
        .json(&map)
        .send();

    match res {
        Ok(res) => {
            let deserialized_result : Result<JsonResponse, reqwest::Error> = res.json();
            match deserialized_result {
                Ok(deserialized) => {
                    if deserialized.success() {
                        println!("success!");
                        println!("deserialized: {:?}", deserialized);
                        // return the key text which is stored as the response msg
                        match deserialized.msg {
                            Some(key) => {
                                // save key to file
                                save_dyndns_key(&key);
                                // save configuration
                                let new_peach_dyndns_config = PeachDynDnsConfig {
                                    domain: domain.to_string(),
                                    dns_server_address: PEACH_DYNDNS_URL.to_string(),
                                    tsig_key_path: TSIG_KEY_PATH.to_string(),
                                    log_file_path: DYNDNS_LOG_PATH.to_string(),
                                };
                                set_peach_dyndns_config(new_peach_dyndns_config);
                                Ok(())
                            },
                            None => Err(PeachDynDnsError::InvalidServerResponse(deserialized))
                        }
                    }
                    else {
                        Err(PeachDynDnsError::ServerError(deserialized))
                    }
                }
                Err(err) => {
                    // deserialization error
                    Err(PeachDynDnsError::ReqwestError(err))
                }
            }
        }
        Err(err) => {
            // reqwest error
            Err(PeachDynDnsError::ReqwestError(err))
        }
    }
}



// main fn for testing
fn main() -> Result<(), PeachDynDnsError> {

    let test_domain = "quartet.dyn.peachcloud.org";

    let result = register_domain(test_domain);

    match result {
        Ok(key) => println!("returned key!"),
        Err(err) => println!("err: {:?}", err)
    }

    Ok(())
}
