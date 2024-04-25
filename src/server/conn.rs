use std::io;
use std::io::Error;
use std::ops::DerefMut;
use std::sync::{Arc, RwLock};
use log::{error, info};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Decoder, Framed};
use crate::server::{CmppDecoder, CmppEncoder, CmppHandler, CmppMessage, IoError};
use crate::server::packet::{Packer, Packet, unpack};
use crate::server::response::Response;

const CMPP_HEADER_LEN: u32 = 12;
const CMPP2_PACKET_MAX: u32 = 2477;
const CMPP2_PACKET_MIN: u32 = 12;
const CMPP3_PACKET_MAX: u32 = 3335;
const CMPP3_PACKET_MIN: u32 = 12;

pub struct ReadBuffer {
    total_len: u32,
    command_id: u32,
    left_data: Vec<u8>
}


pub type Handlers = Vec<Arc<RwLock<dyn CmppHandler>>>;


pub struct Conn {
    pub tcp_stream: TcpStream,
    pub handlers:  Handlers,
}

impl Conn {

    pub async fn serve(&mut self) -> Result<(), IoError> {

        let mut buf = bytes::BytesMut::new();
        let mut decoder = CmppDecoder::default();
        loop {
            match self.tcp_stream.read_buf(&mut buf).await {
                Ok(read_length) => {
                    if read_length == 0 {
                        while let Some(frame) = decoder.decode_eof(&mut buf)? {
                            self.handel_message(frame).await?;
                            continue
                        }
                        return Err(IoError{message: "eof err".to_string()})
                    }

                    while let Some(frame) = decoder.decode(&mut buf)? {
                        self.handel_message(frame).await?;
                        continue
                    }
                }
                Err(e) => {
                    error!("read err: {:?}", e);
                    return Err(IoError{message: format!("read err: {:?}", e).to_string()})
                }
            }


        }


    }

    async fn handel_message(&mut self, msg: CmppMessage) -> Result<(), IoError> {
        let (mut req_packer, res_packer) = unpack(msg.command_id, &msg.body_data)?;
        let seq_id = req_packer.seq_id();
        let mut r = Response{
            packer: res_packer,
            seq_id,
        };

        for h in &self.handlers {
            let rg = h.read().unwrap();
            if let Ok(e) = rg.handle(&mut r) {
                info!("begin hande msg {}", e)
            }
        }


        let rrr = r.packer.pack(seq_id)?;
        self.finish_packet(&rrr).await
    }


     async fn finish_packet(&mut self, res_bytes: &Vec<u8>) -> Result<(), IoError> {
         match self.tcp_stream.write_all(&res_bytes).await {
             Ok(_) => {
                 info!("write len: {}", res_bytes.len());
                 Ok(())
             }
             Err(e) => {
                 return Err(IoError{message: format!("write err: {:?}", e).to_string()})
             }
         }

    }

}