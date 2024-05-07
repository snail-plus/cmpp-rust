use std::net::{SocketAddr};
use std::str::FromStr;
use std::time::Duration;
use crate::server::Result;
use log::{error, info};
use tokio::{io, time};
use tokio::net::{TcpListener, TcpStream};
use super::{Config, Conn};


pub struct Server {
    cfg: Config,
    listener: TcpListener,
}

impl Server {
    pub async fn new(cfg: Config) -> io::Result<Server> {
        let addr = SocketAddr::from_str(&cfg.addr).unwrap();
        let listener = TcpListener::bind(addr).await.unwrap();
        let svr = Server { cfg, listener };
        Ok(svr)
    }

    async fn accept(&mut self) -> Result<TcpStream> {
        let mut backoff = 1;

        // Try to accept a few times
        loop {
            // Perform the accept operation. If a socket is successfully
            // accepted, return it. Otherwise, save the error.
            match self.listener.accept().await {
                Ok((socket, _)) => return Ok(socket),
                Err(err) => {
                    if backoff > 64 {
                        // Accept has failed too many times. Return the error.
                        return Err(err.into());
                    }
                }
            }

            // Pause execution until the back off period elapses.
            time::sleep(Duration::from_secs(backoff)).await;

            // Double the back off
            backoff *= 2;
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        info!("start server, addr: {}", &self.cfg.addr);

        loop {
            let socket = self.accept().await?;
            let client_addr = socket.peer_addr().unwrap().to_string();
            info!("accept client: {}", client_addr.to_string());

            let mut conn = Conn::new(socket);

            tokio::spawn(async move {
                match conn.serve().await {
                    Ok(()) => {
                        info!("client disconnect, client addr: {}", client_addr)
                    }
                    Err(e) => {
                        error!("serve err,exit : {:?}, addr: {}", e, client_addr.to_string())
                    }
                }
            });

        }
    }
}