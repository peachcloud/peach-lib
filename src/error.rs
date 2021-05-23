//! Basic error handling for the network, OLED and stats JSON-RPC clients.
#[derive(Debug)]
pub enum PeachError {
    JsonRpcHttp(jsonrpc_client_http::Error),
    JsonRpcCore(jsonrpc_client_core::Error),
    Serde(serde_json::error::Error),
}

impl From<jsonrpc_client_http::Error> for PeachError {
    fn from(err: jsonrpc_client_http::Error) -> PeachError {
        PeachError::JsonRpcHttp(err)
    }
}

impl From<jsonrpc_client_core::Error> for PeachError {
    fn from(err: jsonrpc_client_core::Error) -> PeachError {
        PeachError::JsonRpcCore(err)
    }
}

impl From<serde_json::error::Error> for PeachError {
    fn from(err: serde_json::error::Error) -> PeachError {
        PeachError::Serde(err)
    }
}
