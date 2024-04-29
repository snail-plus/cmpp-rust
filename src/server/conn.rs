use std::io::Error;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use log::{error, info};
use serde_json::error::Category::Io;
use tokio::{io, time};
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::time::interval;
use tokio_util::codec::{Decoder, Framed};

use crate::server::{CmppDecoder, CmppEncoder, CmppHandler, CmppMessage, Handlers, IoError};
use crate::server::packet::{CmppActiveTestReqPkt, Packer, Packet, unpack};

const CMPP_HEADER_LEN: u32 = 12;
const CMPP2_PACKET_MAX: u32 = 2477;
const CMPP2_PACKET_MIN: u32 = 12;
const CMPP3_PACKET_MAX: u32 = 3335;
const CMPP3_PACKET_MIN: u32 = 12;



static COUNTER: AtomicUsize = AtomicUsize::new(0);

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);


pub struct Conn {
    pub handlers: Handlers,
    rd: ReadHalf<TcpStream>,
    wr: WriteHalf<TcpStream>,
    tx: Sender<Vec<u8>>,
    rx: Receiver<Vec<u8>>,
}

impl Conn {
    pub fn new(stream: TcpStream, handlers: Handlers) -> Self {
        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(32);
        let (mut rd, mut wr) = io::split(stream);
        Conn {
            handlers,
            rd,
            wr,
            tx,
            rx
        }
    }

    pub async fn serve(&mut self) -> Result<(), IoError> {
        let mut buf = bytes::BytesMut::new();
        let mut decoder = CmppDecoder::default();

        loop {
            match self.rd.read_buf(&mut buf).await {
                Ok(read_size) => {
                    if read_size == 0 {
                        return return Err(IoError { message: "eof err".to_string() });
                    }

                    while let Some(frame) = decoder.decode(&mut buf)? {
                        self.handel_message(frame).await?;
                        continue;
                    }
                }
                Err(_) => {
                    return Err(IoError { message: "eof err".to_string() });
                }
            }

        }

    }

    async fn handel_message(&mut self, msg: CmppMessage) -> Result<(), IoError> {
        let msg_count = COUNTER.fetch_add(1, Ordering::Relaxed);
        let (mut req_packer, res_packer) = unpack(msg.command_id, &msg.body_data)?;
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
        self.send_packet(write_bytes).await
    }

    async fn send_packet(&mut self, res_bytes: Vec<u8>) -> Result<(), IoError> {
        if let Err(e) = self.wr.write_all(&res_bytes).await {
            error!("flush err: {:?}", e)
        }

        Ok(())
    }


}