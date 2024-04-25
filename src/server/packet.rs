use std::io::{Error, ErrorKind};
use std::ptr::write;
use log::info;
use serde_json::from_str;

use crate::server::conn::Conn;
use crate::server::response::Response;
use crate::util::byte::u32_to_byte_array;
use crate::util::str::octet_string;
use tokio_util::codec::{Decoder, Encoder, Framed};

const CMPP_CONNECT: u32 = 1;
const CMPP_CONNECT_RESP: u32 = 2147483649;

const CMPP_CONN_REQ_PKT_LEN:  u32 = 4 + 4 + 4 + 6 + 16 + 1 + 4; //39d, 0x27
const CMPP3CONN_RSP_PKT_LEN:  u32 = 4 + 4 + 4 + 4 + 16 + 1;    //33d, 0x21

pub trait Packer: Send {
    fn pack(&self, seq_id: u32) -> Result<Vec<u8>, Error>;
    fn unpack(&self, command_id: u32, data: &Vec<u8>) -> Result<(), Error>;

    fn seq_id(&self) -> u32;
}


pub struct Packet {
    conn: *mut Conn,
    packer: Box<dyn Packer>,
}

impl Packet {
    pub fn new(conn: &mut Conn, packer: Box<dyn Packer>) -> Packet {
        Packet{conn, packer}
    }

}

pub fn unpack(command_id: u32, data: &Vec<u8>) -> Result<(Box<dyn Packer>, Box<dyn Packer>), Error> {
    match command_id {
        CMPP_CONNECT => {
            let mut pkt = CmppConnReqPkt::default();
            pkt.unpack(command_id, data)?;
            info!("receive cmpp_connect");
            Ok((Box::new(pkt), Box::new(Cmpp3ConnRspPkt::default())))
        }
        _ => {
            return Err(Error::from(ErrorKind::InvalidData))
        }
    }
}

pub struct CmppConnReqPkt {
    pub src_addr: String,
    pub auth_src: String,
    pub version: String,
    pub timestamp: u32,
    pub secret: String,
    pub seq_id: u32,
}

impl Default for CmppConnReqPkt {
    fn default() -> CmppConnReqPkt {
        CmppConnReqPkt{
            src_addr: "".to_string(),
            auth_src: "".to_string(),
            version: "".to_string(),
            timestamp: 0,
            secret: "".to_string(),
            seq_id: 0,
        }
    }
}



impl Packer for CmppConnReqPkt {
    fn pack(&self, _seq_id: u32) -> Result<Vec<u8>, Error> {
        let buffer = Vec::with_capacity(CMPP_CONN_REQ_PKT_LEN as usize);
        Ok(buffer)
    }

    fn unpack(&self, _command_id: u32, _data: &Vec<u8>) -> Result<(), Error> {
        return Ok(())
    }

    fn seq_id(&self) -> u32 {
        self.seq_id
    }
}


pub struct Cmpp3ConnRspPkt  {
    status:   u32,
    auth_ismg: String,
    version: u8,
    secret: String,
    auth_src: String,
    seq_id:    u32,
}

impl Default for Cmpp3ConnRspPkt {
    fn default() -> Cmpp3ConnRspPkt {
        Cmpp3ConnRspPkt{
            status: 0,
            auth_ismg: "".to_string(),
            auth_src: "".to_string(),
            secret: "".to_string(),
            seq_id: 0,
            version: 0,
        }
    }
}
impl Packer for Cmpp3ConnRspPkt {
    fn pack(&self, seq_id: u32) -> Result<Vec<u8>, Error> {

        // pack header
        let mut buffer = Vec::with_capacity(CMPP3CONN_RSP_PKT_LEN as usize);
        buffer.extend_from_slice(&u32_to_byte_array(CMPP3CONN_RSP_PKT_LEN));
        buffer.extend_from_slice(&u32_to_byte_array(CMPP_CONNECT_RESP));
        buffer.extend_from_slice(&u32_to_byte_array(seq_id));

        // pack body

        // Status
        buffer.extend_from_slice(&u32_to_byte_array(self.status));

        // auth_msg
        let auth_src = octet_string(String::new(), 16);
        buffer.extend_from_slice(auth_src.as_bytes());

        // Version
        buffer.push(self.version);
        // Timestamp
        buffer.extend_from_slice(&u32_to_byte_array(1234567));

        Ok(buffer)
    }

    fn unpack(&self, _command_id: u32, _data: &Vec<u8>) -> Result<(), Error> {
        Ok(())
    }

    fn seq_id(&self) -> u32 {
        self.seq_id
    }
}