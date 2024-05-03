use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use log::{error, info};
use tokio::io;
use tokio::net::{TcpListener};
use tokio::time::sleep;

use super::{Config, Conn};

#[allow(dead_code)]
const MAX_SIZE: usize = 2048;


pub struct Server {
    cfg: Config,
}

impl Server {
    pub async fn new(cfg: Config) -> io::Result<Server> {
        let svr = Server {cfg};
        Ok(svr)
    }

    pub async fn listen_and_serve(&mut self) {
        let addr = SocketAddr::from_str(&self.cfg.addr).unwrap();
        let tcp = TcpListener::bind(addr).await.unwrap();
        info!("start server, addr: {}, target_addr: {}", &self.cfg.addr, &self.cfg.target_addr);
        loop {
            match tcp.accept().await {
                Ok((stream, client_addr)) => {
                    info!("accept client: {}", client_addr.to_string());

                    tokio::spawn(async move {
                        let mut conn = Conn::new();
                        match conn.serve(stream).await {
                            Ok(()) => {}
                            Err(e) => {
                                error!("serve err,exit : {:?}, addr: {}", e, client_addr.to_string())
                            }
                        }
                    });
                },

                Err(e) => {
                    info!("couldn't get client: {:?}", e);
                    let d = Duration::new(1, 0);
                    sleep(d).await;
                }
            }
        }
    }
    

}