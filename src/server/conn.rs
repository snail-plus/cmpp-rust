use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use bytes::BytesMut;

use log::{error, info};
use tokio::{io, time};
use tokio::io::{AsyncReadExt, AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::time::Interval;
use tokio_util::codec::Decoder;

use crate::server::{CmppDecoder};
use crate::server::cmd::Command;
use crate::server::cmd::deliver::Cmpp3DeliverReqPkt;
use crate::server::Result;


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);


pub struct Conn {
}

impl Conn {
    pub fn new() -> Conn {
        Conn{}
    }

    pub async fn serve(&self, stream: TcpStream) -> Result<()> {

        let mut decoder = CmppDecoder::default();
        let mut buf = BytesMut::with_capacity(1024);
        let (mut reader, mut writer) = io::split(stream);


        info!("buf cap: {}", buf.capacity());

        loop {
            match reader.read_buf(&mut buf).await {
                Ok(read_size) => {
                    if read_size == 0 {
                        return Err("eof".into());
                    }

                    while let Some(mut frame) = decoder.decode(&mut buf)? {
                        let command = Command::parse_frame(frame.command_id, &mut frame.body_data)?;
                        command.apply(&mut writer).await?
                    }
                }
                Err(e) => {
                    return Err(format!("{:?}", e).into());
                }
            }
        }
    }
}

struct MsgInHandler {
    in_rx: Receiver<Command>,
    out_tx: Sender<Command>,
}

impl MsgInHandler {
    async fn run(mut self) {
        while let Some(cmd) = self.in_rx.recv().await {
            info!("IN = {:?}", cmd);
            match cmd {
                Command::Connect(_) => {}
                Command::Submit(submit) => {
                    // 投递状态报告
                    let mut pkt = Cmpp3DeliverReqPkt::new();
                    pkt.seq_id = submit.seq_id;
                    pkt.msg_id = submit.msg_id;
                    if let Err(e) = self.out_tx.send(Command::Deliver(pkt)).await {
                        error!("send deliver err: {:?}", e)
                    }
                }
                _ => {}
            }
        }
    }
}


struct MsgOutHandler {
    out_rx: Receiver<Command>,
    wr: WriteHalf<TcpStream>,
}

impl MsgOutHandler {
    async fn run(mut self) {
        while let Some(cmd) = self.out_rx.recv().await {
            info!("OUT = {:?}", cmd);
            match cmd.into_frame() {
                Ok(res) => {
                    if res.len() == 0 {
                        continue
                    }

                    if let Err(e) = self.wr.write_all(&res).await {
                        error!("write err: {:?}", e);
                        return;
                    }

                    if let Err(e) = self.wr.flush().await {
                        error!("flush err: {:?}", e);
                    }
                }
                Err(e) => {
                    error!("into frame err: {:?}", e)
                }
            }
        }
    }
}


struct IdleHandler {
    last_activity_time: Mutex<Instant>,
}

impl IdleHandler {
    fn update_last_activity_time(&self) {
        let now = Instant::now();
        *self.last_activity_time.lock().unwrap() = now;
    }

    // 检查连接是否空闲
    async fn is_idle(&self, idle_timeout: Duration) -> bool {
        let now = Instant::now();
        let last_activity_time = *self.last_activity_time.lock().unwrap();
        now.duration_since(last_activity_time) > idle_timeout
    }

    // 启动一个定时器来检查空闲状态
    pub async fn start_idle_check(&self, interval: Duration, idle_timeout: Duration) {
        loop {
            time::interval(interval).tick().await;
            if self.is_idle(idle_timeout).await {
                // 处理空闲连接，例如关闭连接或发送心跳
                info!("Connection is idle, closing it...");
                // ... 关闭连接的代码 ...
                break;
            }
        }
    }
}