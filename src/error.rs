//! Basic error handling for the network, OLED, stats and dyndns JSON-RPC clients.
pub use snafu::ResultExt;
use snafu::Snafu;
use std::error;
pub type BoxError = Box<dyn error::Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum PeachError {
    #[snafu(display("{}", source))]
    JsonRpcHttp { source: jsonrpc_client_http::Error },
    #[snafu(display("{}", source))]
    JsonRpcClientCore { source: jsonrpc_client_core::Error },
    #[snafu(display("{}", source))]
    Serde { source: serde_json::error::Error },
    #[snafu(display("{}", source))]
    ParseBoolError { source: std::str::ParseBoolError },
    #[snafu(display("{}", source))]
    SetConfigError { source: serde_yaml::Error },
    #[snafu(display("Failed to read: {}", file))]
    ReadConfigError {
        source: std::io::Error,
        file: String,
    },
    #[snafu(display("Failed to save: {}", file))]
    WriteConfigError {
        source: std::io::Error,
        file: String,
    },
    #[snafu(display("Failed to save tsig key: {} {}", path, source))]
    SaveTsigKeyError {
        source: std::io::Error,
        path: String,
    },
    #[snafu(display("{}", msg))]
    NsUpdateError { msg: String },
    #[snafu(display("Failed to run nsupdate: {}", source))]
    NsCommandError { source: std::io::Error },
    #[snafu(display("Failed to get public IP address: {}", source))]
    GetPublicIpError { source: std::io::Error },
    #[snafu(display("Failed to decode public ip: {}", source))]
    DecodePublicIpError { source: std::str::Utf8Error },
    #[snafu(display("Failed to decode nsupdate output: {}", source))]
    DecodeNsUpdateOutputError { source: std::string::FromUtf8Error },
    #[snafu(display("{}", source))]
    YamlError { source: serde_yaml::Error },
    #[snafu(display("{:?}", err))]
    JsonRpcCore { err: jsonrpc_core::Error },
}

impl From<jsonrpc_client_http::Error> for PeachError {
    fn from(err: jsonrpc_client_http::Error) -> PeachError {
        PeachError::JsonRpcHttp { source: err }
    }
}

impl From<jsonrpc_client_core::Error> for PeachError {
    fn from(err: jsonrpc_client_core::Error) -> PeachError {
        PeachError::JsonRpcClientCore { source: err }
    }
}

impl From<serde_json::error::Error> for PeachError {
    fn from(err: serde_json::error::Error) -> PeachError {
        PeachError::Serde { source: err }
    }
}

impl From<serde_yaml::Error> for PeachError {
    fn from(err: serde_yaml::Error) -> PeachError {
        PeachError::YamlError { source: err }
    }
}

impl From<std::io::Error> for PeachError {
    fn from(err: std::io::Error) -> PeachError {
        PeachError::ReadConfigError {
            source: err,
            file: "".to_string(),
        }
    }
}
