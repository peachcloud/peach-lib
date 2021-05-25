//! Interfaces for writing and reading peachcloud configurations, stored in yaml.
//!
//! The configuration file is located at: "/var/lib/peachcloud/config.yml"

use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

pub const YAML_PATH: &str = "/var/lib/peachcloud/config.yml";

// main type which represents all peachcloud configurations
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PeachConfig {
    peach_dyndns: PeachDynDnsConfig,
    test: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PeachDynDnsConfig {
    pub domain: String,
    pub dns_server_address: String,
    pub tsig_key_path: String,
}

// helper functions for serializing and deserializing PeachConfig from disc
fn save_peach_config(peach_config: PeachConfig) -> Result<PeachConfig, serde_yaml::Error> {
    let yaml_str = serde_yaml::to_string(&peach_config)?;
    println!("{:?}", yaml_str);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(YAML_PATH)
        .expect(&format!("failed to open {}", YAML_PATH));

    writeln!(file, "{}", yaml_str).expect(&format!("failed to write to {}", YAML_PATH));

    Ok(peach_config)
}

fn load_peach_config() -> Result<PeachConfig, serde_yaml::Error> {
    let peach_config_exists = std::path::Path::new(&format!("{}", YAML_PATH)).exists();

    let peach_config: PeachConfig;

    // if this is the first time loading peach_config, we can create a default here
    if !peach_config_exists {
        let peach_dyndns_config = PeachDynDnsConfig {
            domain: "test.dyn.peachcloud.org".to_string(),
            dns_server_address: "dynserver.dyn.peachcloud.org".to_string(),
            tsig_key_path: "/var/lib/peachcloud/peach-dyndns/tsig.key".to_string(),
        };
        peach_config = PeachConfig {
            test: "xyz".to_string(),
            peach_dyndns: peach_dyndns_config,
        };
    }
    // otherwise we load peach config from disk
    else {
        let contents = fs::read_to_string(&format!("{}", YAML_PATH))
            .expect(&format!("failed to read {}", YAML_PATH));
        peach_config = serde_yaml::from_str(&contents)?;
    }

    Ok(peach_config)
}

// interfaces for setting specific config values
pub fn set_peach_dyndns_config(
    new_dyndns_config: PeachDynDnsConfig,
) -> Result<PeachConfig, serde_yaml::Error> {
    let mut peach_config = load_peach_config().unwrap();
    peach_config.peach_dyndns = new_dyndns_config;
    save_peach_config(peach_config)
}

pub fn set_config_test_value(new_test_value: &str) -> Result<PeachConfig, serde_yaml::Error> {
    let mut peach_config = load_peach_config().unwrap();
    peach_config.test = new_test_value.to_string();
    save_peach_config(peach_config)
}