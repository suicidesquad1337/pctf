use std::{collections::HashMap, fmt::Display, str::FromStr, thread::sleep, time::Duration};

use anyhow::Result;
use reqwest::{blocking::Client, Url};
use serde::Deserialize;

/// The host/domian used for openid connect discovery
// const OIDC_HOST: &str = "https://pwnhub-dev.eu.auth0.com/";
/// The id of the oidc client
pub const CLIENT_ID: &str = "XOUgCp9H7k0rkRknnAf8ID6Fz4skI3Wi";
// pub const OAUTH_AUTH: &str = "https://pwnhub-dev.eu.auth0.com/authorize";
pub const DEVICE_AUTH: &str = "https://pwnhub-dev.eu.auth0.com/oauth/device/code";
pub const OAUTH_TOKEN: &str = "https://pwnhub-dev.eu.auth0.com/oauth/token";
pub const AUDIENCE: &str = "http://localhost:8000";

#[derive(Deserialize)]
#[serde(untagged)]
pub enum DeviceAccessTokenResponse {
    Error {
        error: ErrorKind,
        error_description: String,
    },
    Ok {
        access_token: String,
        refresh_token: Option<String>,
        id_token: Option<String>,
        token_type: Option<String>,
        expires_in: u64,
    },
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::AuthorizationPending => {
                write!(f, "The user has not yet approved the authorization")
            }
            ErrorKind::SlowDown => write!(f, "Polling to fast"),
            ErrorKind::ExpiredToken => write!(f, "The authorization took too long"),
            ErrorKind::InvalidGrant => write!(f, "Invalid grant"),
            ErrorKind::AccessDenied => write!(f, "The user rejected the authorization"),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
/// Possibile response error types as per https://auth0.com/docs/authorization/flows/call-your-api-using-the-device-authorization-flow
pub enum ErrorKind {
    AuthorizationPending,
    SlowDown,
    ExpiredToken,
    InvalidGrant,
    AccessDenied,
}

#[non_exhaustive]
#[derive(Deserialize)]
pub struct DeviceAuthorizationRequest {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    pub expires_in: u64,
    pub interval: u64,
}

impl DeviceAuthorizationRequest {
    /// Request a new device authorization code from the oauth2 server
    pub fn new(client: &Client) -> Result<Self> {
        let mut params = HashMap::new();
        params.insert("client_id", CLIENT_ID);
        params.insert("scope", "openid profile");
        params.insert("audience", AUDIENCE);
        client
            .post(Url::from_str(DEVICE_AUTH)?)
            .form(&params)
            .send()?
            .json()
            .map_err(Into::into)
    }

    /// Try polling the access token. This blocks until there's an error or the
    /// access token
    pub fn request(self, client: &Client) -> Result<String> {
        let mut params = HashMap::new();
        params.insert("grant_type", "urn:ietf:params:oauth:grant-type:device_code");
        params.insert("device_code", &self.device_code);
        params.insert("client_id", CLIENT_ID);
        loop {
            let response: DeviceAccessTokenResponse = client
                .post(Url::from_str(OAUTH_TOKEN)?)
                .form(&params)
                .send()?
                .json()?;
            match response {
                DeviceAccessTokenResponse::Ok {
                    access_token,
                    id_token: _,
                    expires_in: _,
                    token_type: _,
                    refresh_token: _,
                } => {
                    return Ok(access_token);
                }
                DeviceAccessTokenResponse::Error {
                    error,
                    error_description,
                } => match error {
                    ErrorKind::AuthorizationPending => sleep(Duration::from_secs(self.interval)),
                    _ => {
                        bail!("Failed to request code ({}: {})", error, error_description)
                    }
                },
            }
        }
    }
}
