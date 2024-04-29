use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::io::Error;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::task::ready;
use std::time;
use std::time::Duration;
use std::vec::Drain;

use chrono::Local;
use log::{debug, error, info};
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex, oneshot};
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;

use crate::server::packet::Packer;
use crate::util::time::format_date;

use super::{Cmpp3SubmitHandler, CmppHandler, CmppLoginHandler, Config, Conn, Handlers, IoError};

#[allow(dead_code)]
const MAX_SIZE: usize = 2048;


pub struct Server {
    counter: Arc<Mutex<i32>>,
    cfg: Config,
    handlers: Vec<Arc<RwLock<dyn CmppHandler>>>
}

impl Server {
    pub async fn new(cfg: Config) -> io::Result<Server> {
        let mut handlers: Handlers = Vec::new();
        handlers.push(Arc::new(RwLock::new(CmppLoginHandler {})));
        handlers.push(Arc::new(RwLock::new(Cmpp3SubmitHandler {})));
        let svr = Server { counter: Arc::new(Mutex::new(0)), cfg, handlers};
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
                    let handlers_clone = self.handlers.clone();
                    
                    tokio::spawn(async move {
                        let mut conn = Conn::new(handlers_clone);
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

    pub fn new_conn(&self,  handlers: Handlers) ->Conn {
         Conn::new(handlers)
    }

}