use anyhow::Result;
use auth::DeviceAuthorizationRequest;
use clap::Parser;
use lazy_static::lazy_static;
use reqwest::blocking::Client;
mod auth;

#[macro_use]
extern crate anyhow;

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
            println!("Waiting for verification ...");
            println!("Done: {}", request.request(&CLIENT)?);
        }
    };
    Ok(())
}
