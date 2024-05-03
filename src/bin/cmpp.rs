use std::env;
use log::{error, info};
use cmpp::server::Config;
use cmpp::server::server::Server;
use tokio::{io, signal};


#[tokio::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let cfg = Config::default();
    let mut srv = Server::new(cfg).await?;

    tokio::select! {
        res = srv.run() => {
            // If an error is received here, accepting connections from the TCP
            // listener failed multiple times and the server is giving up and
            // shutting down.
            //
            // Errors encountered when handling individual connections do not
            // bubble up to this point.
            if let Err(err) = res {
                error!("failed to accept: {:?}", err);
            }
        }
        _ = signal::ctrl_c() => {
            // The shutdown signal has been received.
            info!("shutting down");
        }
    }

    Ok(())
}
