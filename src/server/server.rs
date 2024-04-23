use std::future::Future;
use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use chrono::Local;
use log::{error, info};
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::util::time::format_date;

use super::Config;

#[allow(dead_code)]
const MAX_SIZE: usize = 2048;


pub struct Server {
    counter: Arc<Mutex<i32>>,
    cfg: Config,
}

impl Server {
    pub async fn new(cfg: Config) -> io::Result<Server> {
        let svr = Server { counter: Arc::new(Mutex::new(0)), cfg };
        Ok(svr)
    }

    pub async fn start(&mut self) {
        let addr = SocketAddr::from_str(&self.cfg.addr).unwrap();
        let tcp = TcpListener::bind(addr).await.unwrap();
        info!("start server, addr: {}, target_addr: {}", &self.cfg.addr, &self.cfg.target_addr);
        loop {
            match tcp.accept().await {

                Ok((mut stream, mut addr)) => {
                    info!("客户端addr: {}", addr.to_string());
                    let counter = Arc::clone(&self.counter);

                    tokio::spawn(async move {
                        let mut buf = vec![0u8; 1024];
                        loop {
                            let read_result = stream.read(&mut buf).await.map(|u| {
                                return u;
                            }).map_err(move |e| {
                                error!("failed: {:?}, {}", e, addr.to_string());
                            });

                            if let Err(e) = read_result {
                                return;
                            }

                            let n = stream.read(&mut buf).await.expect("读取失败");
                            if n == 0 {
                                info!("abort conn");
                                return;
                            }

                            let now = Local::now();
                            let date_str = format_date(now, "%Y-%m-%d %H:%M:%S");
                            info!("server time：{}, receive str: {}", date_str, String::from_utf8_lossy(&buf[0..n]).to_string());


                            let mut num =counter.lock().await;
                            *num += 1;

                            let greeting = format!("now: {}, chat time: {}", date_str, num);
                            // release lock
                            drop(num);

                            match stream.write_all(greeting.as_bytes()).await {
                                Ok(_) => {}
                                Err(e) => { error!("write fail {}", e) }
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