//! Interfaces for monitoring and configuring go-sbot using sbotcli.
//!
use crate::error::PeachError;
use serde::{Deserialize, Serialize};
use std::process::Command;

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

/// currently go-sbotcli determines where the working directory is
/// using the home directory of th user that invokes it
/// this could be changed to be supplied as CLI arg
/// but for now all sbotcli commands must first become peach-go-sbot before running
/// the sudoers file is configured to allow this to happen without a password
pub fn sbotcli_command() -> Command {
    let mut command = Command::new("sudo");
    command
        .arg("-u")
        .arg("peach-go-sbot")
        .arg("/usr/bin/sbotcli");
    command
}

pub fn post(msg: &str) -> Result<(), PeachError> {
    let mut command = sbotcli_command();
    let output = command.arg("publish").arg("post").arg(msg).output()?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = std::str::from_utf8(&output.stderr)?;
        Err(PeachError::SbotCliError {
            msg: format!("Error making ssb post: {}", stderr)
        })
    }
}

#[derive(Serialize, Deserialize)]
struct WhoAmIValue {
    id: String,
}

pub fn whoami() -> Result<String, PeachError> {
    let mut command = sbotcli_command();
    let output = command.arg("call").arg("whoami").output()?;
    let text_output = std::str::from_utf8(&output.stdout)?;
    let value: WhoAmIValue = serde_json::from_str(text_output)?;
    let id = value.id;
    Ok(id)
}

pub fn create_invite(uses: i32) -> Result<String, PeachError> {
    let mut command = sbotcli_command();
    let output = command
        .arg("invite")
        .arg("create")
        .arg("--uses")
        .arg(uses.to_string())
        .output()?;
    let text_output = std::str::from_utf8(&output.stdout)?;
    let output = text_output.replace("\n", "");
    Ok(output)
}

pub fn update_pub_name(new_name: &str) -> Result<(), PeachError> {
    let pub_ssb_id = whoami()?;
    let mut command = sbotcli_command();
    let output = command
        .arg("publish")
        .arg("about")
        .arg("--name")
        .arg(new_name)
        .arg(pub_ssb_id)
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = std::str::from_utf8(&output.stderr)?;
        Err(PeachError::SbotCliError {
            msg: format!("Error updating pub name: {}", stderr)
        })
    }
}

pub fn private_message(msg: &str, recipient: &str) -> Result<(), PeachError> {
    let mut command = sbotcli_command();
    let output = command
        .arg("publish")
        .arg("post")
        .arg("--recps")
        .arg(recipient)
        .arg(msg)
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = std::str::from_utf8(&output.stderr)?;
        Err(PeachError::SbotCliError {
            msg: format!("Error sending ssb private message: {}", stderr)
        })
    }
}
