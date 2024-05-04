use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
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
use crate::server::cmd::active::CmppActiveTestReqPkt;
use crate::server::cmd::Command;
use crate::server::cmd::deliver::Cmpp3DeliverReqPkt;
use crate::server::Result;


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);


pub struct Conn {
    seq_id: AtomicUsize,
    interval: Arc<Mutex<Interval>>,
}

impl Conn {
    pub fn new() -> Conn {

        let i = time::interval(HEARTBEAT_INTERVAL);
        Conn {
            interval: Arc::new(Mutex::new(i)),
            seq_id: AtomicUsize::new(0),
        }

    }

    pub async fn serve(&self, stream: TcpStream) -> Result<()> {
        let mut decoder = CmppDecoder::default();
        let mut buf = BytesMut::with_capacity(1024);
        let (mut rd, mut wr) = io::split(stream);

        let (out_tx, out_rx) = mpsc::channel::<Command>(256);
        let (in_tx, in_rx) = mpsc::channel::<Command>(256);

        let out_tx1 = out_tx.clone();
        tokio::spawn(async {
            Conn::handle_msg(in_rx, out_tx1).await;
        });

        tokio::spawn(async move{
            Conn::write_msg(out_rx, &mut wr).await;
        });

        info!("buf cap: {}", buf.capacity());

        loop {
            match rd.read_buf(&mut buf).await {
                Ok(read_size) => {
                    if read_size == 0 {
                       return Err("eof".into());
                    }

                    while let Some(mut frame) = decoder.decode(&mut buf)? {
                        self.interval.lock().unwrap().reset();
                        let in_tx = in_tx.clone();
                        let out_tx = out_tx.clone();
                        let command = Command::parse_frame(frame.command_id, &mut frame.body_data)?;
                        command.apply(in_tx, out_tx).await?
                    }
                }
                Err(e) => {
                    return Err(format!("{:?}", e).into());
                }
            }
        }
    }



    async fn handle_msg(mut in_rx: Receiver<Command>, out_tx: Sender<Command>) {
        while let Some(cmd) = in_rx.recv().await {
            info!("IN = {:?}", cmd);
            match cmd {
                Command::Connect(_) => {}
                Command::Submit(_) => {
                    // 投递状态报告
                    let pkt = Cmpp3DeliverReqPkt::new();
                    let _ = out_tx.send(Command::Deliver(pkt)).await;
                }
                _ => {}
            }
        }
    }

    async fn write_msg(mut out_rx: Receiver<Command>, wh: &mut WriteHalf<TcpStream>) {
        while let Some(cmd) = out_rx.recv().await {
            info!("OUT = {:?}", cmd);
            match cmd.into_frame() {
                Ok(res) => {
                    if let Err(e) = wh.write_all(&res).await {
                        error!("write err: {:?}", e);
                        return;
                    }

                    if let Err(e) = wh.flush().await {
                        error!("flush err: {:?}", e);
                    }
                }
                Err(e) => {
                    error!("into frame err: {:?}", e)
                }
            }
        }
    }

    fn get_seq_id(&self) -> u32 {
        self.seq_id.fetch_add(1, Ordering::Relaxed) as u32
    }

    async fn heartbeat_task(&self, tx: Sender<Command>) {
        let interval = self.interval.clone();
        // 设置心跳定时器
        loop {
            interval.lock().unwrap().tick().await;
            let seq_id = self.get_seq_id();
            // 在这里，我们只是简单地发送心跳数据。在实际应用中，你可能需要处理接收到的消息
            let pkt = CmppActiveTestReqPkt { seq_id};
            let cmd = Command::ActiveTest(pkt);
            if let Err(e) = tx.send(cmd).await {
                let err_str = e.to_string();
                error!("send heartbeat error: {}", err_str);
                return;
            }
        }
    }

    async fn deliver_msg_report() {}
}