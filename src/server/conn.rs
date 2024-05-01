use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use log::{error, info};
use tokio::{io, time};
use tokio::io::{AsyncReadExt, AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio_util::codec::{Decoder};

use crate::server::Result;
use crate::server::{CmppDecoder, CmppMessage, Handlers, IoError};
use crate::server::cmd::Command;
use crate::server::packet::{CmppActiveTestReqPkt, Packer, Packet, unpack};

const CMPP_HEADER_LEN: u32 = 12;
const CMPP2_PACKET_MAX: u32 = 2477;
const CMPP2_PACKET_MIN: u32 = 12;
const CMPP3_PACKET_MAX: u32 = 3335;


static COUNTER: AtomicUsize = AtomicUsize::new(0);

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);


pub struct Conn {
    pub handlers: Handlers,
    seq_id: AtomicUsize
}

impl Conn {
    pub fn new(handlers: Handlers) -> Self {
        Conn {
            handlers,
            seq_id: AtomicUsize::new(0),
        }
    }

    pub async fn serve(&mut self, stream: TcpStream) -> Result<()> {
        let mut buf = bytes::BytesMut::new();
        let mut decoder = CmppDecoder::default();
        let (mut rd, mut wr) = io::split(stream);
        let (msg_tx, mut msg_rx) = mpsc::channel::<Command>(32);


        tokio::spawn(async move {
            while let Some(message) = msg_rx.recv().await {
                info!("GOT = {:?}", message);
            }
        });

        loop {
            match rd.read_buf(&mut buf).await {
                Ok(read_size) => {
                    if read_size == 0 {
                        return Err("EOF error".into());
                    }

                    while let Some(mut frame) = decoder.decode(&mut buf)? {
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

    /*async fn handel_message(&mut self, msg: CmppMessage, tx: Sender<Vec<u8>>) -> Result<(), IoError> {
        let msg_count = COUNTER.fetch_add(1, Ordering::Relaxed);
        let (req_packer, res) = unpack(msg.command_id, &msg.body_data)?;
        if res.is_none() {
            return Ok(());
        }

        let res_packer = res.unwrap();
        info!("receive packer res: {:?}, msg count: {}", req_packer, msg_count);

        let seq_id = req_packer.seq_id();
        let req_packet = Packet {
            packer: req_packer,
            seq_id,
            command_id: msg.command_id,
        };

        let mut res_packet = Packet {
            packer: res_packer,
            seq_id,
            command_id: msg.command_id,
        };


        for h in &self.handlers {
            let rg = h.read().unwrap();
            if rg.support(msg.command_id) {
                rg.handle(&req_packet, &mut res_packet)?;
                break;
            }
        }

        info!("write res: {:?}", res_packet.packer);
        let write_bytes = res_packet.packer.pack(seq_id)?;
        if let Err(e) = tx.send(write_bytes).await {
            return Err(IoError { message: format!("send err: {:?}", e) });
        }

        Ok(())
    }
*/

    async fn heartbeat_task(tx: Sender<Vec<u8>>) {
        let mut interval = time::interval(HEARTBEAT_INTERVAL);
        let mut c = 0;
        // 设置心跳定时器
        loop {
            c += 1;
            interval.tick().await;
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

    async fn deliver_msg_report(mut wr: WriteHalf<TcpStream>, mut rx: Receiver<Vec<u8>>) {

    }

}