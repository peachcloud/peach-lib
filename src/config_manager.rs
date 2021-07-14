//! Interfaces for writing and reading PeachCloud configurations, stored in yaml.
//!
//! Different PeachCloud microservices import peach-lib, so that they can share this interface.
//!
//! The configuration file is located at: "/var/lib/peachcloud/config.yml"

use serde::{Deserialize, Serialize};
use std::fs;
use fslock::LockFile;

use crate::error::PeachError;
use crate::error::*;

// main configuration file
pub const YAML_PATH: &str = "/var/lib/peachcloud/config.yml";

// lock file (used to avoid race conditions during config reading & writing)
pub const LOCK_FILE_PATH: &str = "/var/lib/peachcloud/config.lock";

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
    #[serde(default)] // default is empty vector
    pub ssb_admin_ids: Vec<String>,
}

// helper functions for serializing and deserializing PeachConfig from disc
fn save_peach_config(peach_config: PeachConfig) -> Result<PeachConfig, PeachError> {

    // use a file lock to avoid race conditions while saving config
    let mut lock = LockFile::open(LOCK_FILE_PATH)?;
    lock.lock()?;

    let yaml_str = serde_yaml::to_string(&peach_config)?;

    fs::write(YAML_PATH, yaml_str).context(WriteConfigError {
        file: YAML_PATH.to_string(),
    })?;

    // unlock file lock
    lock.unlock()?;

    // return peach_config
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
            ssb_admin_ids: Vec::new(),
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

pub fn get_peachcloud_domain() -> Result<Option<String>, PeachError> {
    let peach_config = load_peach_config()?;
    if !peach_config.external_domain.is_empty() {
        Ok(Some(peach_config.external_domain))
    } else if !peach_config.dyn_domain.is_empty() {
        Ok(Some(peach_config.dyn_domain))
    } else {
        Ok(None)
    }
}

pub fn set_dyndns_enabled_value(enabled_value: bool) -> Result<PeachConfig, PeachError> {
    let mut peach_config = load_peach_config()?;
    peach_config.dyn_enabled = enabled_value;
    save_peach_config(peach_config)
}

pub fn add_ssb_admin_id(ssb_id: &str) -> Result<PeachConfig, PeachError> {
    let mut peach_config = load_peach_config()?;
    peach_config.ssb_admin_ids.push(ssb_id.to_string());
    save_peach_config(peach_config)
}

pub fn delete_ssb_admin_id(ssb_id: &str) -> Result<PeachConfig, PeachError> {
    let mut peach_config = load_peach_config()?;
    let mut ssb_admin_ids = peach_config.ssb_admin_ids;
    let index_result = ssb_admin_ids.iter().position(|x| *x == ssb_id);
    match index_result {
        Some(index) => {
            ssb_admin_ids.remove(index);
            peach_config.ssb_admin_ids = ssb_admin_ids;
            save_peach_config(peach_config)
        },
        None => {
            Err(PeachError::SsbAdminIdNotFound{ id: ssb_id.to_string()})
        }
    }
}
