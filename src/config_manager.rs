//! Interfaces for writing and reading PeachCloud configurations, stored in yaml.
//!
//! Different PeachCloud microservices import peach-lib, so that they can share this interface.
//!
//! The configuration file is located at: "/var/lib/peachcloud/config.yml"

use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

use crate::error::PeachError;
use crate::error::*;

// main configuration file
pub const YAML_PATH: &str = "/var/lib/peachcloud/config.yml";

// we make use of Serde default values in order to make PeachCloud
// robust and keep running even with a not fully complete config.yml
// main type which represents all peachcloud configurations
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PeachConfig {
    #[serde(default)]
    pub external_domain: String,
    #[serde(default)]
    pub dyn_domain: String,
    #[serde(default)]
    pub dyn_dns_server_address: String,
    #[serde(default)]
    pub dyn_tsig_key_path: String,
    #[serde(default)] // default is false
    pub dyn_enabled: bool,
}

// helper functions for serializing and deserializing PeachConfig from disc
fn save_peach_config(peach_config: PeachConfig) -> Result<PeachConfig, PeachError> {
    let yaml_str = serde_yaml::to_string(&peach_config)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(YAML_PATH)
        .context(ReadConfigError {
            file: YAML_PATH.to_string(),
        })?;

    writeln!(file, "{}", yaml_str).context(WriteConfigError {
        file: YAML_PATH.to_string(),
    })?;

    Ok(peach_config)
}

pub fn load_peach_config() -> Result<PeachConfig, PeachError> {
    let peach_config_exists = std::path::Path::new(YAML_PATH).exists();

    let peach_config: PeachConfig;

    // if this is the first time loading peach_config, we can create a default here
    if !peach_config_exists {
        peach_config = PeachConfig {
            external_domain: "".to_string(),
            dyn_domain: "".to_string(),
            dyn_dns_server_address: "".to_string(),
            dyn_tsig_key_path: "".to_string(),
            dyn_enabled: false,
        };
    }
    // otherwise we load peach config from disk
    else {
        let contents = fs::read_to_string(YAML_PATH).context(ReadConfigError {
            file: YAML_PATH.to_string(),
        })?;
        peach_config = serde_yaml::from_str(&contents)?;
    }

    Ok(peach_config)
}

// interfaces for setting specific config values
pub fn set_peach_dyndns_config(
    dyn_domain: &str,
    dyn_dns_server_address: &str,
    dyn_tsig_key_path: &str,
    dyn_enabled: bool,
) -> Result<PeachConfig, PeachError> {
    let mut peach_config = load_peach_config()?;
    peach_config.dyn_domain = dyn_domain.to_string();
    peach_config.dyn_dns_server_address = dyn_dns_server_address.to_string();
    peach_config.dyn_tsig_key_path = dyn_tsig_key_path.to_string();
    peach_config.dyn_enabled = dyn_enabled;
    save_peach_config(peach_config)
}

pub fn set_external_domain(new_external_domain: &str) -> Result<PeachConfig, PeachError> {
    let mut peach_config = load_peach_config()?;
    peach_config.external_domain = new_external_domain.to_string();
    save_peach_config(peach_config)
}

pub fn set_dyndns_enabled_value(enabled_value: bool) -> Result<PeachConfig, PeachError> {
    let mut peach_config = load_peach_config()?;
    peach_config.dyn_enabled = enabled_value;
    save_peach_config(peach_config)
}
