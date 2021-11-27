use anyhow::Result;
use clap::Parser;
use lazy_static::lazy_static;
use reqwest::blocking::Client;

use crate::oauth2::DeviceAuthorizationRequest;

mod oauth2;

// Constaints that allow compiling with cargo-install
/// The host/domian used for openid connect discovery
// const OIDC_HOST: &str = "https://pwnhub-dev.eu.auth0.com/";
/// The id of the oidc client
pub const CLIENT_ID: &str = "XOUgCp9H7k0rkRknnAf8ID6Fz4skI3Wi";
// pub const OAUTH_AUTH: &str = "https://pwnhub-dev.eu.auth0.com/authorize";
pub const DEVICE_AUTH: &str = "https://pwnhub-dev.eu.auth0.com/oauth/device/code";
pub const OAUTH_TOKEN: &str = "https://pwnhub-dev.eu.auth0.com/oauth/token";
pub const AUDIENCE: &str = "http://localhost:8000";

#[derive(Parser)]
enum Commands {
    /// Login using discord
    Login,
}

fn main() -> Result<()> {
    let cmd = Commands::parse();
    lazy_static! {
        static ref CLIENT: Client = Client::new();
    }
    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match cmd {
        Commands::Login => {
            let request = DeviceAuthorizationRequest::new(&CLIENT)?;
            println!(
                "Please follow this link: {}\nYour code is {}",
                request.verification_uri_complete, request.user_code
            );
            println!("Waiting ...");
            let response = request.poll(&CLIENT)?;
            println!("Done!\nYour access token is {}", response.access_token);
        }
    };
    Ok(())
}
