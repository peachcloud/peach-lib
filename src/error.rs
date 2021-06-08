//! Basic error handling for the network, OLED, stats and dyndns JSON-RPC clients.
#[derive(Debug)]
pub enum PeachError {
    JsonRpcHttp(jsonrpc_client_http::Error),
    JsonRpcClientCore(jsonrpc_client_core::Error),
    Serde(serde_json::error::Error),
    ParseBoolError(std::str::ParseBoolError),
    SetConfigError(serde_yaml::Error),
    NsUpdateError(String),
    YamlError(serde_yaml::Error),
    JsonRpcCore(jsonrpc_core::Error),
}

impl From<jsonrpc_client_http::Error> for PeachError {
    fn from(err: jsonrpc_client_http::Error) -> PeachError {
        PeachError::JsonRpcHttp(err)
    }
}

impl From<jsonrpc_client_core::Error> for PeachError {
    fn from(err: jsonrpc_client_core::Error) -> PeachError {
        PeachError::JsonRpcClientCore(err)
    }
}

impl From<serde_json::error::Error> for PeachError {
    fn from(err: serde_json::error::Error) -> PeachError {
        PeachError::Serde(err)
    }
}

impl From<serde_yaml::Error> for PeachError {
    fn from(err: serde_yaml::Error) -> PeachError {
        PeachError::YamlError(err)
    }
}
