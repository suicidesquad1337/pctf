//! An wrapper for the oauth2 [Device Authorization Grant][1]
//!
//! [1]: https://datatracker.ietf.org/doc/html/rfc8628

mod error;
use std::{
    collections::HashMap, result::Result as StdResult, str::FromStr, thread::sleep, time::Duration,
};

use anyhow::Result;
#[doc(inline)]
pub use error::PollResponseError;
use reqwest::{blocking::Client, Url};
use serde::Deserialize;

use crate::{AUDIENCE, CLIENT_ID, DEVICE_AUTH, OAUTH_TOKEN};

#[non_exhaustive]
#[derive(Deserialize, Debug)]
/// A Device Authorization Request as defined in [RFC 8628 section 3.1][1]
///
/// The server will return an url that the user should follow to authorize this
/// application
/// [1]: https://datatracker.ietf.org/doc/html/rfc8628#section-3.1
pub struct DeviceAuthorizationRequest {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    pub expires_in: u64,
    pub interval: u64,
}

#[derive(Debug, Deserialize)]
pub struct DeviceAccessTokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: u64,
}

impl DeviceAuthorizationRequest {
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

    pub fn poll(self, client: &Client) -> Result<DeviceAccessTokenResponse> {
        let mut params = HashMap::new();
        params.insert("grant_type", "urn:ietf:params:oauth:grant-type:device_code");
        params.insert("device_code", &self.device_code);
        params.insert("client_id", CLIENT_ID);
        loop {
            #[derive(Deserialize)]
            #[serde(untagged)]
            /// Enum to determine if a [`DeviceAuthorizationRequest`] was successful or not.
            /// This way, we can return a [`std::result::Result`] directly
            enum ResponseMatcher {
                Ok(DeviceAccessTokenResponse),
                Err(PollResponseError),
            }
            let response: ResponseMatcher = client
                .post(Url::from_str(OAUTH_TOKEN)?)
                .form(&params)
                .send()?
                .json()?;
            match response {
                ResponseMatcher::Ok(d) => return StdResult::Ok(d),
                ResponseMatcher::Err(e) => match e {
                    PollResponseError::AuthorizationPending {
                        error_description: _,
                    } => sleep(Duration::from_secs(self.interval)),
                    _ => return StdResult::Err(e.into()),
                },
            }
        }
    }
}
