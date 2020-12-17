use std::env;

use jsonrpc_client_core::{expand_params, jsonrpc_client};
use jsonrpc_client_http::HttpTransport;
use log::{debug, info};

use crate::error::PeachError;

/// Creates a JSON-RPC client with http transport and calls the `peach-oled`
/// `clear` method.
pub fn clear() -> std::result::Result<(), PeachError> {
    debug!("Creating HTTP transport for OLED client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_OLED_SERVER").unwrap_or_else(|_| "127.0.0.1:5112".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_oled service.");
    let mut client = PeachOledClient::new(transport_handle);

    client.clear().call()?;
    debug!("Cleared the OLED display.");

    Ok(())
}

/// Creates a JSON-RPC client with http transport and calls the `peach-oled`
/// `draw` method.
///
/// # Arguments
///
/// * `bytes` - A Vec of 8 byte unsigned int.
/// * `width` - A 32 byte unsigned int.
/// * `height` - A 32 byte unsigned int.
/// * `x_coord` - A 32 byte signed int.
/// * `y_coord` - A 32 byte signed int.
pub fn draw(
    bytes: Vec<u8>,
    width: u32,
    height: u32,
    x_coord: i32,
    y_coord: i32,
) -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for OLED client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_OLED_SERVER").unwrap_or_else(|_| "127.0.0.1:5112".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_oled service.");
    let mut client = PeachOledClient::new(transport_handle);

    client.draw(bytes, width, height, x_coord, y_coord).call()?;
    debug!("Drew to the OLED display.");

    Ok("success".to_string())
}

/// Creates a JSON-RPC client with http transport and calls the `peach-oled`
/// `flush` method.
pub fn flush() -> std::result::Result<(), PeachError> {
    debug!("Creating HTTP transport for OLED client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_OLED_SERVER").unwrap_or_else(|_| "127.0.0.1:5112".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_oled service.");
    let mut client = PeachOledClient::new(transport_handle);

    client.flush().call()?;
    debug!("Flushed the OLED display.");

    Ok(())
}

/// Creates a JSON-RPC client with http transport and calls the `peach-oled`
/// `ping` method.
pub fn ping() -> std::result::Result<(), PeachError> {
    debug!("Creating HTTP transport for OLED client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_OLED_SERVER").unwrap_or_else(|_| "127.0.0.1:5112".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_oled service.");
    let mut client = PeachOledClient::new(transport_handle);

    client.ping().call()?;
    debug!("Pinged the OLED microservice.");

    Ok(())
}

/// Creates a JSON-RPC client with http transport and calls the `peach-oled`
/// `power` method.
///
/// # Arguments
///
/// * `power` - A boolean expression
pub fn power(on: bool) -> std::result::Result<(), PeachError> {
    debug!("Creating HTTP transport for OLED client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_OLED_SERVER").unwrap_or_else(|_| "127.0.0.1:5112".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_oled service.");
    let mut client = PeachOledClient::new(transport_handle);

    client.power(on).call()?;
    debug!("Toggled the OLED display power.");

    Ok(())
}

/// Creates a JSON-RPC client with http transport and calls the `peach-oled`
/// `draw` method.
///
/// # Arguments
///
/// * `x_coord` - A 32 byte signed int.
/// * `y_coord` - A 32 byte signed int.
/// * `string` - A reference to a string slice
/// * `font_size` - A reference to a string slice
pub fn write(
    x_coord: i32,
    y_coord: i32,
    string: &str,
    font_size: &str,
) -> std::result::Result<String, PeachError> {
    debug!("Creating HTTP transport for OLED client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_OLED_SERVER").unwrap_or_else(|_| "127.0.0.1:5112".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_oled service.");
    let mut client = PeachOledClient::new(transport_handle);

    client.write(x_coord, y_coord, string, font_size).call()?;
    debug!("Wrote to the OLED display.");

    Ok("success".to_string())
}

jsonrpc_client!(pub struct PeachOledClient {
    /// Creates a JSON-RPC request to clear the OLED display.
    pub fn clear(&mut self) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to draw to the OLED display.
    pub fn draw(&mut self, bytes: Vec<u8>, width: u32, height: u32, x_coord: i32, y_coord: i32) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to flush the OLED display.
    pub fn flush(&mut self) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to ping the OLED microservice.
    pub fn ping(&mut self) -> RpcRequest<String>;

/// Creates a JSON-RPC request to toggle the power of the OLED display.
    pub fn power(&mut self, on: bool) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to write to the OLED display.
    pub fn write(&mut self, x_coord: i32, y_coord: i32, string: &str, font_size: &str) -> RpcRequest<String>;
});
