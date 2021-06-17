//! Interfaces for monitoring and configuring go-sbot using sbotcli.
//!
//! In the future this could be moved to its own rust repo, but starting here.
use crate::error::PeachError;
use std::process::{Command, Stdio};

pub fn is_sbot_online() -> Result<bool, PeachError> {
    let output = Command::new("/usr/bin/systemctl")
        .arg("status")
        .arg("peach-go-sbot")
        .output()?;
    let status = output.status;
    // returns true if the service had an exist status of 0 (is running)
    let is_running = status.success();
    Ok(is_running)
}
