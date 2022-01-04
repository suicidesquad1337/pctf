//! Typed errors for the oauth2 [Device Authorization Grant][1]
//!
//! [1]: https://datatracker.ietf.org/doc/html/rfc8628
use serde::Deserialize;
use thiserror::Error;

macro_rules! poll_response_error {
    (
        $(
            $(#[$attr:meta])*
            $v:ident
        ),*$(,)?
    ) => {
        #[derive(Debug, Error, Deserialize)]
        #[serde(tag = "error")]
        #[serde(rename_all = "snake_case")]
        /// An error type that represents the possible error types defined in
        /// [RFC 8628 section 7.3][1]
        ///
        /// [1]: https://datatracker.ietf.org/doc/html/rfc8628#section-7.3
        pub enum PollResponseError {
            $(
                #[error("{error_description}")]
                $(#[$attr])*
                $v {
                    error_description: String,
                },
            )*
        }
    };
}

// documentation for variants taken from
// https://datatracker.ietf.org/doc/html/rfc8628#section-3.5
// and https://datatracker.ietf.org/doc/html/rfc6749#section-5.2
// note that only invalid_grant is taken from rfc 6749
poll_response_error! {
    /// authorization_pending
    ///
    /// The authorization request is still pending. The client SHOULD repeat
    /// the access token request to the token endpoint (a process known as
    /// polling).
    AuthorizationPending,
    /// slow_down
    ///
    ///  A variant of "authorization_pending", the authorization request is
    /// still pending and polling should continue, but the interval MUST
    /// be increased by 5 seconds for this and all subsequent requests.
    SlowDown,
    /// expired_token
    ///
    /// The "device_code" has expired, and the device authorization
    /// session has concluded.  The client MAY commence a new device
    /// authorization request but SHOULD wait for user interaction before
    /// restarting to avoid unnecessary polling.
    ExpiredToken,
    /// invalid_grant
    ///
    /// Defined as part of the standard oauth2 errors in [RFC 6749 section 5.2][1]:
    /// The provided authorization grant (e.g., authorization
    /// code, resource owner credentials) or refresh token is
    /// invalid, expired, revoked, does not match the redirection
    /// URI used in the authorization request, or was issued to
    /// another client.
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc6749#section-5.2
    InvalidGrant,
    /// access_denied
    ///
    /// The authorization request was denied.
    AccessDenied,
}

#[cfg(test)]
mod tests {
    use super::PollResponseError;

    #[allow(non_upper_case_globals)]
    #[test]
    fn parse_errors() {
        const authorization_pending: &str = r#"
        {
            "error": "authorization_pending",
            "error_description": "User did not approve the request yet"
        }
        "#;
        const slow_down: &str = r#"
        {
            "error": "slow_down",
            "error_description": "Please poll slower"
        }
        "#;
        const expired_token: &str = r#"
        {
            "error": "expired_token",
            "error_description": "Token expired"
        }
        "#;
        const invalid_grant: &str = r#"
        {
            "error": "invalid_grant",
            "error_description": "Token is invalid"
        }
        "#;
        const access_denied: &str = r#"
        {
            "error": "access_denied",
            "error_description": "User denied the request"
        }
        "#;

        let ap: PollResponseError = serde_json::from_str(authorization_pending).unwrap();
        assert!(matches!(
            ap,
            PollResponseError::AuthorizationPending {
                error_description: _
            }
        ));

        let sd: PollResponseError = serde_json::from_str(slow_down).unwrap();
        println!("{:#?}", sd);
        assert!(matches!(
            sd,
            PollResponseError::SlowDown {
                error_description: _
            }
        ));

        let et: PollResponseError = serde_json::from_str(expired_token).unwrap();
        assert!(matches!(
            et,
            PollResponseError::ExpiredToken {
                error_description: _
            }
        ));

        let ig: PollResponseError = serde_json::from_str(invalid_grant).unwrap();
        assert!(matches!(
            ig,
            PollResponseError::InvalidGrant {
                error_description: _
            }
        ));

        let ad: PollResponseError = serde_json::from_str(access_denied).unwrap();
        assert!(matches!(
            ad,
            PollResponseError::AccessDenied {
                error_description: _
            }
        ));
    }
}
