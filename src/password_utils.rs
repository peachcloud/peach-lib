use crate::config_manager::get_peachcloud_domain;
use crate::error::PeachError;
use crate::error::StdIoError;
use crate::sbot_client;
use log::info;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use snafu::ResultExt;
use std::iter;
use std::process::Command;

/// filepath where nginx basic auth passwords are stored
pub const HTPASSWD_FILE: &str = "/var/lib/peachcloud/passwords/htpasswd";
/// filepath where random temporary password is stored for password resets
pub const HTPASSWD_TEMPORARY_PASSWORD_FILE: &str =
    "/var/lib/peachcloud/passwords/temporary_password";
/// the username of the user for nginx basic auth
pub const PEACHCLOUD_AUTH_USER: &str = "admin";

/// Returns Ok(()) if the supplied password is correct,
/// and returns Err if the supplied password is incorrect.
pub fn verify_password(password: &str) -> Result<(), PeachError> {
    let output = Command::new("/usr/bin/htpasswd")
        .arg("-vb")
        .arg(HTPASSWD_FILE)
        .arg(PEACHCLOUD_AUTH_USER)
        .arg(password)
        .output()
        .context(StdIoError {
            msg: "htpasswd is not installed",
        })?;
    if output.status.success() {
        Ok(())
    } else {
        Err(PeachError::InvalidPassword)
    }
}

/// Checks if the given passwords are valid, and returns Ok() if they are and
/// a PeachError otherwise.
/// Currently this just checks that the passwords are the same,
/// but could be extended to test if they are strong enough.
pub fn validate_new_passwords(new_password1: &str, new_password2: &str) -> Result<(), PeachError> {
    if new_password1 == new_password2 {
        Ok(())
    } else {
        Err(PeachError::PasswordsDoNotMatch)
    }
}

/// Uses htpasswd to set a new password for the admin user
pub fn set_new_password(new_password: &str) -> Result<(), PeachError> {
    let output = Command::new("/usr/bin/htpasswd")
        .arg("-cb")
        .arg(HTPASSWD_FILE)
        .arg(PEACHCLOUD_AUTH_USER)
        .arg(new_password)
        .output()
        .context(StdIoError {
            msg: "htpasswd is not installed",
        })?;
    if output.status.success() {
        Ok(())
    } else {
        let err_output = String::from_utf8(output.stderr)?;
        Err(PeachError::FailedToSetNewPassword { msg: err_output })
    }
}

/// Uses htpasswd to set a new temporary password for the admin user
/// which can be used to reset the permanent password
pub fn set_new_temporary_password(new_password: &str) -> Result<(), PeachError> {
    let output = Command::new("/usr/bin/htpasswd")
        .arg("-cb")
        .arg(HTPASSWD_TEMPORARY_PASSWORD_FILE)
        .arg(PEACHCLOUD_AUTH_USER)
        .arg(new_password)
        .output()
        .context(StdIoError {
            msg: "htpasswd is not installed",
        })?;
    if output.status.success() {
        Ok(())
    } else {
        let err_output = String::from_utf8(output.stderr)?;
        Err(PeachError::FailedToSetNewPassword { msg: err_output })
    }
}

/// Returns Ok(()) if the supplied temp_password is correct,
/// and returns Err if the supplied temp_password is incorrect
pub fn verify_temporary_password(password: &str) -> Result<(), PeachError> {
    // TODO: confirm temporary password has not expired
    let output = Command::new("/usr/bin/htpasswd")
        .arg("-vb")
        .arg(HTPASSWD_TEMPORARY_PASSWORD_FILE)
        .arg(PEACHCLOUD_AUTH_USER)
        .arg(password)
        .output()
        .context(StdIoError {
            msg: "htpasswd is not installed",
        })?;
    if output.status.success() {
        Ok(())
    } else {
        Err(PeachError::InvalidPassword)
    }
}

/// generates a temporary password and sends it via ssb dm
/// to the ssb id configured to be the admin of the peachcloud device
pub fn send_password_reset() -> Result<(), PeachError> {
    // first generate a new random password of ascii characters
    let mut rng = thread_rng();
    let temporary_password: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(10)
        .collect();
    // save this string as a new temporary password
    set_new_temporary_password(&temporary_password)?;
    let domain = get_peachcloud_domain()?;

    // then send temporary password as a private ssb message to admin
    let mut msg = format!(
        "Your new temporary password is: {}

If you are on the same WiFi network as your PeachCloud device you can reset your password \
using this link: http://peach.local/reset_password",
        temporary_password
    );
    // if there is an external domain, then include remote link in message
    // otherwise dont include it
    let remote_link = match domain {
        Some(domain) => {
            format!(
                "\n\nOr if you are on a different WiFi network, you can reset your password \
            using the the following link: {}/reset_password",
                domain
            )
        }
        None => "".to_string(),
    };
    msg += &remote_link;
    // finally send the message to the admin
    let ssb_admin_id = "@LZx+HP6/fcjUm7vef2eaBKAQ9gAKfzmrMVGzzdJiQtA=.ed25519";
    info!("msg: {:?}", msg);
    sbot_client::private_message(&msg, ssb_admin_id)?;
    Ok(())
}
