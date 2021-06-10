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
use crate::error::*;
use jsonrpc_client_core::{expand_params, jsonrpc_client};
use jsonrpc_client_http::HttpTransport;
use log::{debug, info};
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
            save_dyndns_key(&key);
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
                Err(err) => Err(PeachError::ParseBoolError { source: err }),
            }
        }
        Err(err) => Err(PeachError::JsonRpcClientCore { source: err }),
    }
}

/// Helper function to get public ip address of PeachCloud device.
fn get_public_ip_address() -> String {
    // TODO: consider other ways to get public IP address
    let output = Command::new("/usr/bin/curl")
        .arg("ifconfig.me")
        .output()
        .expect("failed to get public IP");
    let command_output = std::str::from_utf8(&output.stdout).expect("Incorrect format");
    command_output.to_string()
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
            .stdin(Stdio::piped
                ())
            .spawn().context(NsCommandError)?;
        // pass nsupdate commands via stdin
        let public_ip_address = get_public_ip_address();
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
            .expect("failed to wait on child");
        info!("output: {:?}", nsupdate_output);
        // We only return a successful result if nsupdate was successful
        if nsupdate_output.status.success() {
            info!("nsupdate succeeded, returning ok");
            Ok(true)
        } else {
            info!("nsupdate failed, returning error");
            let err_msg = String::from_utf8(nsupdate_output.stdout)
                .expect("failed to read stdout from nsupdate");
            Err(PeachError::NsUpdateError { msg: err_msg })
        }
    }
}

jsonrpc_client!(pub struct PeachDynDnsClient {
    pub fn register_domain(&mut self, domain: &str) -> RpcRequest<String>;
    pub fn is_domain_available(&mut self, domain: &str) -> RpcRequest<String>;
});
