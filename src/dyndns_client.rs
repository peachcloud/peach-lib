//! Client which makes jsonrpc requests via HTTP to the `peach-dyndns-server` API which runs on the peach-vps.
//! Note this is the one service in peach-lib which makes requests to an external server off of the local device.
//!
//! If the requests are successful, dyndns configurations are saved locally on the PeachCloud device,
//! which are then used by the peach-dyndns-cronjob to update the dynamic IP using nsupdate.
//!
//! There is also one function in this file, dyndns_update_ip, which doesn't interact with the jsonrpc server.
//! This function uses nsupdate to actually update dns records directly.
//!
//! The domain for dyndns updates is stored in /var/lib/peachcloud/config.yml
//! The tsig key for authenticating the updates is stored in /var/lib/peachcloud/peach-dyndns/tsig.key
use crate::config_manager::{load_peach_config, set_peach_dyndns_config};
use crate::error::PeachError;
use crate::error::{
    ChronoParseError, DecodeNsUpdateOutputError, DecodePublicIpError, GetPublicIpError,
    NsCommandError, SaveDynDnsResultError, SaveTsigKeyError,
};
use regex::Regex;
use chrono::prelude::*;
use jsonrpc_client_core::{expand_params, jsonrpc_client};
use jsonrpc_client_http::HttpTransport;
use log::{debug, info};
use snafu::ResultExt;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::str::ParseBoolError;

/// constants for dyndns configuration
pub const PEACH_DYNDNS_URL: &str = "http://dynserver.dyn.peachcloud.org";
pub const TSIG_KEY_PATH: &str = "/var/lib/peachcloud/peach-dyndns/tsig.key";
pub const PEACH_DYNDNS_CONFIG_PATH: &str = "/var/lib/peachcloud/peach-dyndns";
pub const DYNDNS_LOG_PATH: &str = "/var/lib/peachcloud/peach-dyndns/latest_result.log";

/// helper function which saves dyndns TSIG key returned by peach-dyndns-server to /var/lib/peachcloud/peach-dyndns/tsig.key
pub fn save_dyndns_key(key: &str) -> Result<(), PeachError> {
    // create directory if it doesn't exist
    fs::create_dir_all(PEACH_DYNDNS_CONFIG_PATH).context(SaveTsigKeyError {
        path: PEACH_DYNDNS_CONFIG_PATH.to_string(),
    })?;
    // write key text
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(TSIG_KEY_PATH)
        .context(SaveTsigKeyError {
            path: TSIG_KEY_PATH.to_string(),
        })?;
    writeln!(file, "{}", key).context(SaveTsigKeyError {
        path: TSIG_KEY_PATH.to_string(),
    })?;
    Ok(())
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
    info!("Creating client for peach-dyndns service.");
    let mut client = PeachDynDnsClient::new(transport_handle);

    info!("Performing register_domain call to peach-dyndns-server");
    let res = client.register_domain(&domain).call();
    match res {
        Ok(key) => {
            // save new TSIG key
            save_dyndns_key(&key)?;
            // save new configuration values
            let set_config_result =
                set_peach_dyndns_config(domain, PEACH_DYNDNS_URL, TSIG_KEY_PATH, true);
            match set_config_result {
                Ok(_) => {
                    let response = "success".to_string();
                    Ok(response)
                }
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(PeachError::JsonRpcClientCore { source: err }),
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
            let result: Result<bool, ParseBoolError> = FromStr::from_str(&result_str);
            match result {
                Ok(result_bool) => Ok(result_bool),
                Err(err) => Err(PeachError::PeachParseBoolError { source: err }),
            }
        }
        Err(err) => Err(PeachError::JsonRpcClientCore { source: err }),
    }
}

/// Helper function to get public ip address of PeachCloud device.
fn get_public_ip_address() -> Result<String, PeachError> {
    // TODO: consider other ways to get public IP address
    let output = Command::new("/usr/bin/curl")
        .arg("ifconfig.me")
        .output()
        .context(GetPublicIpError)?;
    let command_output = std::str::from_utf8(&output.stdout).context(DecodePublicIpError)?;
    Ok(command_output.to_string())
}

/// Reads dyndns configurations from config.yml
/// and then uses nsupdate to update the IP address for the configured domain
pub fn dyndns_update_ip() -> Result<bool, PeachError> {
    info!("Running dyndns_update_ip");
    let peach_config = load_peach_config()?;
    info!(
        "Using config:
    dyn_tsig_key_path: {:?}
    dyn_domain: {:?}
    dyn_dns_server_address: {:?}
    dyn_enabled: {:?}
    ",
        peach_config.dyn_tsig_key_path,
        peach_config.dyn_domain,
        peach_config.dyn_dns_server_address,
        peach_config.dyn_enabled,
    );
    if !peach_config.dyn_enabled {
        info!("dyndns is not enabled, not updating");
        Ok(false)
    } else {
        // call nsupdate passing appropriate configs
        let nsupdate_command = Command::new("/usr/bin/nsupdate")
            .arg("-k")
            .arg(peach_config.dyn_tsig_key_path)
            .arg("-v")
            .stdin(Stdio::piped())
            .spawn()
            .context(NsCommandError)?;
        // pass nsupdate commands via stdin
        let public_ip_address = get_public_ip_address()?;
        info!("found public ip address: {}", public_ip_address);
        let ns_commands = format!(
            "
        server {NAMESERVER}
        zone {ZONE}
        update delete {DOMAIN} A
        update add {DOMAIN} 30 A {PUBLIC_IP_ADDRESS}
        send",
            NAMESERVER = "ns.peachcloud.org",
            ZONE = peach_config.dyn_domain,
            DOMAIN = peach_config.dyn_domain,
            PUBLIC_IP_ADDRESS = public_ip_address,
        );
        write!(nsupdate_command.stdin.as_ref().unwrap(), "{}", ns_commands).unwrap();
        let nsupdate_output = nsupdate_command
            .wait_with_output()
            .context(NsCommandError)?;
        info!("output: {:?}", nsupdate_output);
        // We only return a successful result if nsupdate was successful
        if nsupdate_output.status.success() {
            info!("nsupdate succeeded, returning ok");
            // log a timestamp that the update was successful
            log_successful_nsupdate()?;
            // return true
            Ok(true)
        } else {
            info!("nsupdate failed, returning error");
            let err_msg =
                String::from_utf8(nsupdate_output.stdout).context(DecodeNsUpdateOutputError)?;
            Err(PeachError::NsUpdateError { msg: err_msg })
        }
    }
}

// Helper function to log a timestamp of the latest successful nsupdate
pub fn log_successful_nsupdate() -> Result<bool, PeachError> {
    let now_timestamp = chrono::offset::Utc::now().to_rfc3339();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(DYNDNS_LOG_PATH)
        .context(SaveDynDnsResultError)?;
    write!(file, "{}", now_timestamp).context(SaveDynDnsResultError)?;
    Ok(true)
}

/// Helper function to return how many seconds since peach-dyndns-updater successfully ran
pub fn get_num_seconds_since_successful_dns_update() -> Result<Option<i64>, PeachError> {
    let log_exists = std::path::Path::new(DYNDNS_LOG_PATH).exists();
    if !log_exists {
        Ok(None)
    } else {
        let contents =
            fs::read_to_string(DYNDNS_LOG_PATH).expect("Something went wrong reading the file");
        // replace newline if found
        let contents = contents.replace("\n", "");
        let time_ran_dt = DateTime::parse_from_rfc3339(&contents).context(ChronoParseError {
            msg: "Error parsing dyndns time from latest_result.log".to_string(),
        })?;
        let current_time: DateTime<Utc> = Utc::now();
        let duration = current_time.signed_duration_since(time_ran_dt);
        let duration_in_seconds = duration.num_seconds();
        Ok(Some(duration_in_seconds))
    }
}

/// helper function which returns a true result if peach-dyndns-updater is enabled
/// and has successfully run recently (in the last six minutes)
pub fn is_dns_updater_online() -> Result<bool, PeachError> {
    // first check if it is enabled in peach-config
    let peach_config = load_peach_config()?;
    let is_enabled = peach_config.dyn_enabled;
    // then check if it has successfully run within the last 6 minutes (60*6 seconds)
    let num_seconds_since_successful_update = get_num_seconds_since_successful_dns_update()?;
    let ran_recently: bool;
    match num_seconds_since_successful_update {
        Some(seconds) => {
            ran_recently = seconds < (60 * 6);
        }
        // if the value is None, then the last time it ran successfully is unknown
        None => {
            ran_recently = false;
        }
    }
    // debug log
    info!("is_dyndns_enabled: {:?}", is_enabled);
    info!("dyndns_ran_recently: {:?}", ran_recently);
    // if both are true, then return true
    Ok(is_enabled && ran_recently)
}

/// helper function which builds a full dynamic dns domain from a subdomain
pub fn get_full_dynamic_domain(subdomain: &str) -> String {
    format!("{}.dyn.peachcloud.org", subdomain)
}

/// helper function to get a dyndns subdomain from a dyndns full domain
pub fn get_dyndns_subdomain(dyndns_full_domain: &str) -> Option<String> {
    let re = Regex::new(r"(.*)\.dyn\.peachcloud\.org").ok()?;
    let caps = re.captures(dyndns_full_domain)?;
    let subdomain = caps.get(1).map_or("", |m| m.as_str());
    Some(subdomain.to_string())
}

// helper function which checks if a dyndns domain is new
pub fn check_is_new_dyndns_domain(dyndns_full_domain: &str) -> bool {
    let peach_config = load_peach_config().unwrap();
    let previous_dyndns_domain = peach_config.dyn_domain;
    dyndns_full_domain != previous_dyndns_domain
}

jsonrpc_client!(pub struct PeachDynDnsClient {
    pub fn register_domain(&mut self, domain: &str) -> RpcRequest<String>;
    pub fn is_domain_available(&mut self, domain: &str) -> RpcRequest<String>;
});
