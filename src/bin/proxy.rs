use std::env;
use hello::server::Config;
use hello::server::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let cfg = Config::default();

    let mut srv = Server::new(cfg).await?;
    srv.listen_and_serve().await;
    Ok(())
}
