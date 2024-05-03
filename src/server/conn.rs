use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

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
use crate::server::Result;

const CMPP_HEADER_LEN: u32 = 12;
const CMPP2_PACKET_MAX: u32 = 2477;
const CMPP2_PACKET_MIN: u32 = 12;
const CMPP3_PACKET_MAX: u32 = 3335;


static COUNTER: AtomicUsize = AtomicUsize::new(0);

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);


pub struct Conn {
    seq_id: AtomicUsize,
    interval: Arc<Mutex<Interval>>,
    buf: bytes::BytesMut,
}

impl Conn {
    pub fn new() -> Self {
        let i = time::interval(HEARTBEAT_INTERVAL);
        Conn {
            interval: Arc::new(Mutex::new(i)),
            seq_id: AtomicUsize::new(0),
            buf: bytes::BytesMut::new(),
        }
    }

    pub async fn serve(&mut self, stream: TcpStream) -> Result<()> {
        let mut decoder = CmppDecoder::default();
        let (mut rd, mut wr) = io::split(stream);
        let (msg_tx, mut msg_rx) = mpsc::channel::<Command>(32);
        let i = self.interval.clone();

        tokio::spawn(async move {
            while let Some(message) = msg_rx.recv().await {
                info!("GOT = {:?}", message);
            }
        });

        loop {
            match rd.read_buf(&mut self.buf).await {
                Ok(read_size) => {
                    if read_size == 0 {
                        return Err("EOF error".into());
                    }

                    while let Some(mut frame) = decoder.decode(&mut self.buf)? {
                        i.lock().unwrap().reset();
                        let tx = msg_tx.clone();
                        let command = Command::parse_frame(frame.command_id, &mut frame.body_data)?;
                        command.apply(tx, &mut wr).await?
                    }
                }
                Err(e) => {
                    return Err(format!("{:?}", e).into());
                }
            }
        }

    }


    async fn heartbeat_task(&self, tx: Sender<Vec<u8>>) {
        let mut interval = self.interval.clone();
        let mut c = 0;
        // 设置心跳定时器
        loop {
            c += 1;
            interval.lock().unwrap().tick().await;
            // 在这里，我们只是简单地发送心跳数据。在实际应用中，你可能需要处理接收到的消息
            let pkt = CmppActiveTestReqPkt { seq_id: 0 };
            if let Err(e) = tx.send(pkt.pack(c).unwrap()).await {
                let err_str = e.to_string();
                error!("send heartbeat error: {}", err_str);
                return;
            }
        }
    }

    async fn flush_task(mut wr: WriteHalf<TcpStream>, mut rx: Receiver<Vec<u8>>) {
        while let Some(msg) = rx.recv().await {
            if msg.len() == 0 {
                return;
            }

            if let Err(e) = wr.write_all(&msg).await {
                error!("send heartbeat error: {:?}", e);
                continue;
            }
        }
    }

    async fn deliver_msg_report() {

    }

}