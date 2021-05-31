//! Interfaces for writing and reading PeachCloud configurations, stored in yaml.
//!
//! Different PeachCloud microservices import peach-lib, so that they can share this interface.
//!
//! The configuration file is located at: "/var/lib/peachcloud/config.yml"

use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

// main configuration file
pub const YAML_PATH: &str = "/var/lib/peachcloud/config.yml";

// we make use of Serde default values in order to make PeachCloud
// robust and keep running even with a not fully complete config.yml

// main type which represents all peachcloud configurations
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PeachConfig {
    #[serde(default = "default_dyndns_config")]
    pub peach_dyndns: PeachDynDnsConfig,
    #[serde(default)]
    pub test: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PeachDynDnsConfig {
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub dns_server_address: String,
    #[serde(default)]
    pub tsig_key_path: String,
    #[serde(default)] // default is false
    pub enabled: bool,
}

pub fn default_dyndns_config() -> PeachDynDnsConfig {
    PeachDynDnsConfig {
        domain: "".to_string(),
        dns_server_address: "".to_string(),
        tsig_key_path: "".to_string(),
        enabled: false,
    }
}

// helper functions for serializing and deserializing PeachConfig from disc
fn save_peach_config(peach_config: PeachConfig) -> Result<PeachConfig, serde_yaml::Error> {
    let yaml_str = serde_yaml::to_string(&peach_config)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(YAML_PATH)
        .unwrap_or_else(|_| panic!("failed to open {}", YAML_PATH));

    writeln!(file, "{}", yaml_str).unwrap_or_else(|_| panic!("failed to write to {}", YAML_PATH));

    Ok(peach_config)
}

pub fn load_peach_config() -> Result<PeachConfig, serde_yaml::Error> {
    let peach_config_exists = std::path::Path::new(YAML_PATH).exists();

    let peach_config: PeachConfig;

    // if this is the first time loading peach_config, we can create a default here
    if !peach_config_exists {
        let peach_dyndns_config = PeachDynDnsConfig {
            domain: "test.dyn.peachcloud.org".to_string(),
            dns_server_address: "dynserver.dyn.peachcloud.org".to_string(),
            tsig_key_path: "/var/lib/peachcloud/peach-dyndns/tsig.key".to_string(),
            enabled: false,
        };
        peach_config = PeachConfig {
            test: "xyz".to_string(),
            peach_dyndns: peach_dyndns_config,
        };
    }
    // otherwise we load peach config from disk
    else {
        let contents = fs::read_to_string(YAML_PATH)
            .unwrap_or_else(|_| panic!("failed to read {}", YAML_PATH));
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
