//! Perform JSON-RPC calls to the `peach-network` microservice.
//!
//! This module contains a JSON-RPC client and associated data structures for
//! making calls to the `peach-network` microservice. Each RPC has a
//! corresponding method which creates an HTTP transport, makes the call to the
//! RPC microservice and returns the response to the caller. These convenience
//! methods simplify the process of performing RPC calls from other modules.
//!
//! Several helper methods are also included here which bundle multiple client
//! calls to achieve the desired functionality.

use std::env;

use jsonrpc_client_core::{jsonrpc_client, expand_params};
use jsonrpc_client_http::HttpTransport;
use log::{debug, info};
use serde::{Deserialize, Serialize};

use crate::error::PeachError;
use crate::stats_client::Traffic;

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessPoint {
    pub detail: Option<Scan>,
    pub signal: Option<i32>,
    pub state: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Networks {
    pub ssid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Scan {
    pub protocol: String,
    pub frequency: String,
    pub signal_level: String,
    pub ssid: String,
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `activate_ap` method.
pub fn activate_ap() -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.activate_ap().call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `activate_client` method.
pub fn activate_client() -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.activate_client().call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `add_wifi` method.
///
/// # Arguments
///
/// * `ssid` - A string slice containing the SSID of an access point.
/// * `pass` - A string slice containing the password for an access point.
pub fn add(ssid: &str, pass: &str) -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.add(ssid, pass).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `available_networks` method, which returns a list of in-range access points.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
pub fn available_networks(iface: &str) -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.available_networks(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `connect` method, which disables other network connections and enables the
/// connection for the chosen network, identified by ID and interface.
///
/// # Arguments
///
/// * `id` - A string slice containing a network identifier.
/// * `iface` - A string slice containing the network interface identifier.
pub fn connect(id: &str, iface: &str) -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.connect(id, iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `id` method.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
/// * `ssid` - A string slice containing the SSID of a network.
pub fn id(iface: &str, ssid: &str) -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.id(iface, ssid).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `ip` method.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
pub fn ip(iface: &str) -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.ip(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `ping` method, which serves as a means of determining availability of the
/// microservice (ie. there will be no response if `peach-network` is not
/// running).
pub fn ping() -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);
    let response = client.ping().call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `reconfigure` method.
pub fn reconfigure() -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.reconfigure().call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `rssi` method.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
pub fn rssi(iface: &str) -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.rssi(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `rssi_percent` method.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
pub fn rssi_percent(iface: &str) -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.rssi_percent(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `saved_networks` method, which returns a list of networks saved in
/// `wpa_supplicant.conf`.
pub fn saved_networks() -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);
    let response = client.saved_networks().call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `ssid` method.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
pub fn ssid(iface: &str) -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.ssid(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `state` method.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
pub fn state(iface: &str) -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.state(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `status` method.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
pub fn status(iface: &str) -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.status(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `traffic` method.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
pub fn traffic(iface: &str) -> std::result::Result<Traffic, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.traffic(iface).call()?;
    let t: Traffic = serde_json::from_str(&response).unwrap();

    Ok(t)
}

/// Helper function to determine if a given SSID already exists in the
/// `wpa_supplicant.conf` file, indicating that network credentials have already
/// been added for that access point. Creates a JSON-RPC client with http
/// transport and calls the `peach-network` `saved_networks` method. Returns a
/// boolean expression inside a Result type.
///
/// # Arguments
///
/// * `ssid` - A string slice containing the SSID of a network.
pub fn saved_ap(ssid: &str) -> std::result::Result<bool, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    // retrieve a list of access points with saved credentials
    let saved_aps = match client.saved_networks().call() {
        Ok(ssids) => {
            let networks: Vec<Networks> = serde_json::from_str(ssids.as_str())
                .expect("Failed to deserialize saved_networks response");
            networks
        }
        // return an empty vector if there are no saved access point credentials
        Err(_) => Vec::new(),
    };

    // loop through the access points in the list
    for network in saved_aps {
        // return true if the access point ssid matches the given ssid
        if network.ssid == ssid {
            return Ok(true);
        }
    }

    // return false if no matches are found
    Ok(false)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `id` and `disable` methods.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
/// * `ssid` - A string slice containing the SSID of a network.
pub fn disable(iface: &str, ssid: &str) -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    info!("Performing id call to peach-network microservice.");
    let id = client.id(&iface, &ssid).call()?;
    info!("Performing disable call to peach-network microservice.");
    client.disable(&id, &iface).call()?;

    let response = "success".to_string();

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `id`, `delete` and `save` methods.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
/// * `ssid` - A string slice containing the SSID of a network.
pub fn forget(iface: &str, ssid: &str) -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    info!("Performing id call to peach-network microservice.");
    let id = client.id(&iface, &ssid).call()?;
    info!("Performing delete call to peach-network microservice.");
    // WEIRD BUG: the parameters below are technically in the wrong order:
    // it should be id first and then iface, but somehow they get twisted.
    // i don't understand computers.
    client.delete(&iface, &id).call()?;
    info!("Performing save call to peach-network microservice.");
    client.save().call()?;

    let response = "success".to_string();

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `id`, `delete`, `save` and `add` methods. These combined calls allow the
/// saved password for an access point to be updated.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
/// * `ssid` - A string slice containing the SSID of a network.
/// * `pass` - A string slice containing the password for a network.
pub fn update(
    iface: &str,
    ssid: &str,
    pass: &str,
) -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    // get the id of the network
    info!("Performing id call to peach-network microservice.");
    let id = client.id(&iface, &ssid).call()?;
    // delete the old credentials
    // WEIRD BUG: the parameters below are technically in the wrong order:
    // it should be id first and then iface, but somehow they get twisted.
    // i don't understand computers.
    info!("Performing delete call to peach-network microservice.");
    client.delete(&iface, &id).call()?;
    // save the updates to wpa_supplicant.conf
    info!("Performing save call to peach-network microservice.");
    client.save().call()?;
    // add the new credentials
    info!("Performing add call to peach-network microservice.");
    client.add(ssid, pass).call()?;
    // reconfigure wpa_supplicant with latest addition to config
    info!("Performing reconfigure call to peach-network microservice.");
    client.reconfigure().call()?;

    let response = "success".to_string();

    Ok(response)
}

jsonrpc_client!(pub struct PeachNetworkClient {
    /// JSON-RPC request to activate the access point.
    pub fn activate_ap(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to activate the wireless client (wlan0).
    pub fn activate_client(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to add credentials for an access point.
    pub fn add(&mut self, ssid: &str, pass: &str) -> RpcRequest<String>;

    /// JSON-RPC request to list all networks in range of the given interface.
    pub fn available_networks(&mut self, iface: &str) -> RpcRequest<String>;

    /// JSON-RPC request to connect the network for the given interface and ID.
    pub fn connect(&mut self, id: &str, iface: &str) -> RpcRequest<String>;

    /// JSON-RPC request to delete the credentials for the given network from the wpa_supplicant config.
    pub fn delete(&mut self, id: &str, iface: &str) -> RpcRequest<String>;

    /// JSON-RPC request to disable the network for the given interface and ID.
    pub fn disable(&mut self, id: &str, iface: &str) -> RpcRequest<String>;

    /// JSON-RPC request to disconnect the network for the given interface.
    //pub fn disconnect(&mut self, iface: &str) -> RpcRequest<String>;

    /// JSON-RPC request to get the ID for the given interface and SSID.
    pub fn id(&mut self, iface: &str, ssid: &str) -> RpcRequest<String>;

    /// JSON-RPC request to get the IP address for the given interface.
    pub fn ip(&mut self, iface: &str) -> RpcRequest<String>;

    /// JSON-RPC request to set a new network password for the given interface and ID.
    //pub fn modify(&mut self, id: &str, iface: &str, pass: &str) -> RpcRequest<String>;

    /// JSON-RPC request to check peach-network availability.
    pub fn ping(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to reread the wpa_supplicant config for the given interface.
    pub fn reconfigure(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to reconnect WiFi for the given interface.
    //pub fn reconnect(&mut self, iface: &str) -> RpcRequest<String>;

    /// JSON-RPC request to get the average signal strength (dBm) for the given interface.
    pub fn rssi(&mut self, iface: &str) -> RpcRequest<String>;

    /// JSON-RPC request to get the average signal quality (%) for the given interface.
    pub fn rssi_percent(&mut self, iface: &str) -> RpcRequest<String>;

    /// JSON-RPC request to save network configuration updates to file.
    pub fn save(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to list all networks saved in `wpa_supplicant.conf`.
    pub fn saved_networks(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to get the SSID of the currently-connected network for the given interface.
    pub fn ssid(&mut self, iface: &str) -> RpcRequest<String>;

    /// JSON-RPC request to get the state for the given interface.
    pub fn state(&mut self, iface: &str) -> RpcRequest<String>;

    /// JSON-RPC request to get the status of the given interface.
    pub fn status(&mut self, iface: &str) -> RpcRequest<String>;

    /// JSON-RPC request to get the network traffic for the given interface.
    pub fn traffic(&mut self, iface: &str) -> RpcRequest<String>;
});
