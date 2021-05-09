//! Helpers for connecting to Docker and health-checking that connection.

use bollard::Docker;
use bollard::errors::Error as BollardError;

/// Attempts to establish a connection to a local Docker instance and sends a Ping
/// request to check if the server is accessible.
///
/// On UNIX-based systems the default socket at `/var/run/docker.sock` will be used,
/// whereas Windows machines will utilize a named pipe.
pub async fn connect_local() -> Result<Docker, BollardError> {
    let socket = Docker::connect_with_local_defaults()?;
    socket.ping().await?; // Make sure the server is reachable.

    Ok(socket)
}
