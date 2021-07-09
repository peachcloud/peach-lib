// this is to ignore a clippy warning that suggests
// to replace code with the same code that is already there (possibly a bug)
#![allow(clippy::nonstandard_macro_braces)]

pub mod config_manager;
pub mod dyndns_client;
pub mod error;
pub mod network_client;
pub mod oled_client;
pub mod password_utils;
pub mod sbot_client;
pub mod stats_client;

// re-export error types
pub use jsonrpc_client_core;
pub use jsonrpc_core;
pub use serde_json;
pub use serde_yaml;
