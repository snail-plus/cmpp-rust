use std::fmt::Debug;
use std::io::{Error, ErrorKind};

use bytes::{Buf, BufMut};
use tokio_util::codec::{Decoder, Encoder};

use crate::util::byte::{u32_to_byte_array, u64_to_byte_array};
use crate::util::str::{oct_string, octet_string, ucs2_to_utf8};

pub const CMPP_CONNECT: u32 = 1;
pub const CMPP_CONNECT_RESP: u32 = 2147483649;
pub const CMPP_SUBMIT: u32 = 4;
pub const CMPP_SUBMIT_RESP: u32 = 2147483652;

pub const CMPP_ACTIVE_TEST_REQ_PKT_LEN: u32 = 12;

pub const  CMPP_ACTIVE_TEST: u32= 8;

pub const CMPP_HEADER_LEN: u32 = 12;

const CMPP_CONN_REQ_PKT_LEN: u32 = 4 + 4 + 4 + 6 + 16 + 1 + 4;
//39d, 0x27
const CMPP3CONN_RSP_PKT_LEN: u32 = 4 + 4 + 4 + 4 + 16 + 1;    //33d, 0x21

pub trait Packer: Send + Debug {
    fn pack(&self, seq_id: u32) -> Result<Vec<u8>, Error>;
    fn unpack(&mut self, data: &Vec<u8>) -> Result<(), Error>;

    fn seq_id(&self) -> u32;
}


pub fn unpack(command_id: u32, data: &Vec<u8>) -> Result<(Box<dyn Packer>, Box<dyn Packer>), Error> {
    match command_id {
        CMPP_CONNECT => {
            let mut pkt = CmppConnReqPkt::default();
            pkt.unpack(data)?;
            Ok((Box::new(pkt), Box::new(Cmpp3ConnRspPkt::default())))
        }

        CMPP_SUBMIT => {
            let mut pkt = Cmpp3SubmitReqPkt::default();
            pkt.unpack(data)?;
            let msg_id = pkt.msg_id;
            let seq_id = pkt.seq_id;
            Ok((Box::new(pkt), Box::new(Cmpp3SubmitRspPkt{
                msg_id,
                result: 0,
                seq_id,
            })))
        }

        _ => {
            return Err(Error::from(ErrorKind::InvalidData));
        }
    }
}

#[derive(Debug)]
pub struct Packet {
    pub packer: Box<dyn Packer>,
    pub seq_id: u32,
    pub command_id: u32,
}

#[derive(Debug)]
pub struct CmppActiveTestReqPkt {
    // session info
    pub(crate) seq_id: u32
}

impl Packer for CmppActiveTestReqPkt {

    fn pack(&self, seq_id: u32) -> Result<Vec<u8>, Error> {
        let mut buffer = Vec::with_capacity(12usize);
        buffer.put_u32(12u32);
        buffer.put_u32(CMPP_ACTIVE_TEST);
        buffer.put_u32(seq_id);
        Ok(buffer)
    }

    fn unpack(&mut self, data: &Vec<u8>) -> Result<(), Error> {
        return Ok(());
    }

    fn seq_id(&self) -> u32 {
        self.seq_id
    }
}

#[derive(Debug)]
pub struct CmppActiveTestRspPkt  {
    reserved: u8,
    // session info
    seq_id: u32
}

impl Packer for CmppActiveTestRspPkt {
    fn pack(&self, _seq_id: u32) -> Result<Vec<u8>, Error> {
        let buffer = Vec::with_capacity(CMPP_CONN_REQ_PKT_LEN as usize);
        Ok(buffer)
    }

    fn unpack(&mut self, _data: &Vec<u8>) -> Result<(), Error> {
        return Ok(());
    }

    fn seq_id(&self) -> u32 {
        self.seq_id
    }
}

#[derive(Debug)]
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
        CmppConnReqPkt {
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

    fn unpack(&mut self, _data: &Vec<u8>) -> Result<(), Error> {
        return Ok(());
    }

    fn seq_id(&self) -> u32 {
        self.seq_id
    }
}

#[derive(Debug)]
pub struct Cmpp3ConnRspPkt {
    status: u32,
    auth_ismg: String,
    version: u8,
    secret: String,
    auth_src: String,
    seq_id: u32,
}

impl Default for Cmpp3ConnRspPkt {
    fn default() -> Cmpp3ConnRspPkt {
        Cmpp3ConnRspPkt {
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

        Ok(buffer)
    }

    fn unpack(&mut self, _data: &Vec<u8>) -> Result<(), Error> {
        Ok(())
    }

    fn seq_id(&self) -> u32 {
        self.seq_id
    }
}

#[derive(Debug)]
pub struct Cmpp3SubmitReqPkt {
    pub msg_id: u64,
    pub pk_total: u8,
    pub pk_number: u8,
    pub registered_delivery: u8,
    pub msg_level: u8,
    pub service_id: String,
    pub fee_user_type: u8,
    pub fee_terminal_id: String,
    pub fee_terminal_type: u8,
    pub tp_pid: u8,
    pub tp_udhi: u8,
    pub msg_fmt: u8,
    pub msg_src: String,
    pub fee_type: String,
    pub fee_code: String,
    pub valid_time: String,
    pub at_time: String,
    pub src_id: String,
    pub dest_usr_tl: u8,
    pub dest_terminal_id: Vec<String>,
    pub dest_terminal_type: u8,
    msg_length: u8,
    msg_content: String,
    link_id: String,

    // session info
    seq_id: u32,
}

impl Default for Cmpp3SubmitReqPkt {
    fn default() -> Cmpp3SubmitReqPkt {
        Cmpp3SubmitReqPkt {
            msg_id: 0,
            pk_total: 0,
            pk_number: 0,
            registered_delivery: 0,
            msg_level: 0,
            service_id: "".to_string(),
            fee_user_type: 0,
            fee_terminal_id: "".to_string(),
            fee_terminal_type: 0,
            tp_pid: 0,
            tp_udhi: 0,
            msg_fmt: 0,
            msg_src: "".to_string(),
            fee_type: "".to_string(),
            fee_code: "".to_string(),
            valid_time: "".to_string(),
            at_time: "".to_string(),
            src_id: "".to_string(),
            dest_usr_tl: 0,
            dest_terminal_id: vec![],
            dest_terminal_type: 0,
            msg_length: 0,
            msg_content: "".to_string(),
            link_id: "".to_string(),
            seq_id: 0,
        }
    }
}


impl Packer for Cmpp3SubmitReqPkt {
    fn pack(&self, _seq_id: u32) -> Result<Vec<u8>, Error> {
        Ok(vec![])
    }

    fn unpack(&mut self, data: &Vec<u8>) -> Result<(), Error> {
        let mut buf = bytes::BytesMut::with_capacity(data.len());
        buf.extend_from_slice(data);
        // Sequence Id
        self.seq_id = buf.get_u32();
        // msg_id
        self.msg_id = buf.get_u64();
        self.pk_total = buf.get_u8();
        self.pk_number = buf.get_u8();

        self.registered_delivery = buf.get_u8();
        self.msg_level = buf.get_u8();

        let mut service_id_vec = vec![0u8; 10];
        buf.copy_to_slice(&mut service_id_vec);
        self.service_id = oct_string(service_id_vec);

        self.fee_user_type = buf.get_u8();

        let mut fee_terminal_id_vec = vec![0u8; 32];
        buf.copy_to_slice(&mut fee_terminal_id_vec);
        self.fee_terminal_id = oct_string(fee_terminal_id_vec);

        self.fee_terminal_type = buf.get_u8();

        self.tp_pid = buf.get_u8();
        self.tp_udhi = buf.get_u8();
        self.msg_fmt = buf.get_u8();

        let mut msg_src_vec = vec![0u8; 6];
        buf.copy_to_slice(&mut msg_src_vec);
        self.msg_src = oct_string(msg_src_vec);

        let mut fee_type_vec = vec![0u8; 2];
        buf.copy_to_slice(&mut fee_type_vec);
        self.fee_type = oct_string(fee_type_vec);

        let mut fee_code_vec = vec![0u8; 6];
        buf.copy_to_slice(&mut fee_code_vec);
        self.fee_code = oct_string(fee_code_vec);

        let mut valid_time_vec = vec![0u8; 17];
        buf.copy_to_slice(&mut valid_time_vec);
        self.valid_time = oct_string(valid_time_vec);

        let mut at_time_vec = vec![0u8; 17];
        buf.copy_to_slice(&mut at_time_vec);
        self.at_time = oct_string(at_time_vec);

        let mut src_id_vec = vec![0u8; 21];
        buf.copy_to_slice(&mut src_id_vec);
        self.src_id = oct_string(src_id_vec);

        self.dest_usr_tl = buf.get_u8();
        let mut dest_terminal_ids = Vec::with_capacity(self.dest_usr_tl as usize);
        for _i in 0..self.dest_usr_tl {
            let mut dest_terminal_id_vec = vec![0u8; 32];
            buf.copy_to_slice(&mut dest_terminal_id_vec);
            dest_terminal_ids.push(oct_string(dest_terminal_id_vec));
        }
        self.dest_terminal_id = dest_terminal_ids;

        self.dest_terminal_type = buf.get_u8();

        self.msg_length = buf.get_u8();
        let mut msg_content_vec = vec![0u8; self.msg_length as usize];
        buf.copy_to_slice(&mut msg_content_vec);
        match ucs2_to_utf8(msg_content_vec.as_slice()) {
            Ok(content) => { self.msg_content = content }
            Err(e) => { return Err(Error::new(ErrorKind::Other, format!("解析msg_content失败 {:?}", e))); }
        }

        let mut link_id_vec = vec![0u8; 20];
        buf.copy_to_slice(&mut link_id_vec);
        self.link_id = oct_string(link_id_vec);

        Ok(())
    }

    fn seq_id(&self) -> u32 {
        self.seq_id
    }
}

#[derive(Debug)]
pub struct Cmpp3SubmitRspPkt  {
    msg_id: u64,
    result: u32,

    // session info
    seq_id: u32,
}

impl Packer for Cmpp3SubmitRspPkt {
    fn pack(&self, seq_id: u32) -> Result<Vec<u8>, Error> {
        let pkt_len = CMPP_HEADER_LEN + 8 + 4;
        let mut buffer = Vec::with_capacity(pkt_len as usize);
        // Pack header

        buffer.extend_from_slice(&u32_to_byte_array(pkt_len));
        buffer.extend_from_slice(&u32_to_byte_array(CMPP_SUBMIT_RESP));
        buffer.extend_from_slice(&u32_to_byte_array(seq_id));

        // Pack Body
        buffer.extend_from_slice(&u64_to_byte_array(self.msg_id));
        buffer.extend_from_slice(&u32_to_byte_array(self.result));
        Ok(buffer)
    }

    fn unpack(&mut self, _data: &Vec<u8>) -> Result<(), Error> {
        Ok(())
    }

    fn seq_id(&self) -> u32 {
        self.seq_id
    }
}