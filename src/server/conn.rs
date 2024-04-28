use std::{io, task};
use std::io::Error;
use std::ops::DerefMut;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Poll, Wake, Waker};
use std::time::Duration;
use log::{error, info};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::time;
use tokio::time::interval;
use tokio_util::codec::{Decoder, Framed};
use crate::server::{CmppDecoder, CmppEncoder, CmppHandler, CmppMessage, IoError};
use crate::server::packet::{CmppActiveTestReqPkt, Packer, Packet, unpack};

const CMPP_HEADER_LEN: u32 = 12;
const CMPP2_PACKET_MAX: u32 = 2477;
const CMPP2_PACKET_MIN: u32 = 12;
const CMPP3_PACKET_MAX: u32 = 3335;
const CMPP3_PACKET_MIN: u32 = 12;

pub type Handlers = Vec<Arc<RwLock<dyn CmppHandler>>>;

const COUNTER: AtomicUsize = AtomicUsize::new(0);

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);

struct MyWaker {}

impl Wake for MyWaker {
    fn wake(self: Arc<Self>) {}
}

pub struct Conn {
    pub tcp_stream: TcpStream,
    pub handlers: Handlers,
}

impl Conn {
    pub fn new(stream: TcpStream, handlers: Handlers) -> Self {
        Conn {
            tcp_stream: stream,
            handlers,
        }
    }

    pub async fn serve(&mut self) -> Result<(), IoError> {
        let mut buf = bytes::BytesMut::new();
        let mut decoder = CmppDecoder::default();
        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(32);
        let mut heartbeat_ticker = interval(HEARTBEAT_INTERVAL);


        loop {
            let mut seq_id = 0;
            tokio::select! {
                biased;
                read_result = self.tcp_stream.read_buf(&mut buf) => {
                    heartbeat_ticker.reset();
                    match read_result {
                         Ok(_) => {
                            while let Some(frame) = decoder.decode(&mut buf)? {
                                 self.handel_message(frame).await?;
                                 continue;
                            }
                        },
                         Err(_) => {
                            return Err(IoError { message: "eof err".to_string() });
                        }
                    }

                },

                _ = heartbeat_ticker.tick() => {

                    let req = CmppActiveTestReqPkt { seq_id };
                    if let Err(e) = self.tcp_stream.write_all(&req.pack(seq_id).unwrap()).await {
                       return Err(IoError { message: format!("write err: {:?}", e).to_string() });
                    }
                }


            }
        }

    }

    async fn handel_message(&mut self, msg: CmppMessage) -> Result<(), IoError> {
        COUNTER.fetch_add(1, Ordering::Relaxed);
        let (mut req_packer, res_packer) = unpack(msg.command_id, &msg.body_data)?;

        let msg_count = COUNTER.load(Ordering::Relaxed);
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
        self.finish_packet(&write_bytes).await
    }

    async fn finish_packet(&mut self, res_bytes: &Vec<u8>) -> Result<(), IoError> {
        match self.tcp_stream.write_all(&res_bytes).await {
            Ok(_) => {
                Ok(())
            }
            Err(e) => {
                return Err(IoError { message: format!("write err: {:?}", e).to_string() });
            }
        }
    }
}