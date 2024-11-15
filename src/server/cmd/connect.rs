use bytes::{Buf, BufMut};

use crate::server::cmd::{CMPP3CONN_RSP_PKT_LEN, CMPP_CONNECT_RESP};
use crate::server::Result;
use crate::util::str::{oct_string, octet_string};

#[derive(Debug, Clone)]
pub struct CmppConnReqPkt {
    pub src_addr: String,
    pub auth_src: Vec<u8>,
    pub version: u8,
    pub timestamp: u32,
    pub secret: String,
    pub seq_id: u32,
}

impl CmppConnReqPkt {

    fn new() -> CmppConnReqPkt {
        CmppConnReqPkt {
            src_addr: "".to_string(),
            auth_src: vec![],
            version: 0,
            timestamp: 0,
            secret: "".to_string(),
            seq_id: 0,
        }
    }

    pub(crate) fn parse_frame(_seq_id: u32, data: &mut Vec<u8>) -> Result<CmppConnReqPkt>{
        let mut pkt = CmppConnReqPkt::new();

        let mut buf = bytes::BytesMut::with_capacity(data.len());
        buf.extend_from_slice(data);

        // src_addr
        let mut src_addr_vec = vec![0u8; 6];
        buf.copy_to_slice(&mut src_addr_vec);
        pkt.src_addr = oct_string(src_addr_vec);
        // AuthSrc
        let mut auth_src_vec = vec![0u8; 16];
        buf.copy_to_slice(&mut auth_src_vec);
        pkt.auth_src = auth_src_vec;
        // version
        pkt.version = buf.get_u8();
        // timestamp
        pkt.timestamp = buf.get_u32();
        Ok(pkt)
    }

    pub(crate)  fn apply(&self) -> Result<Cmpp3ConnRspPkt> {
        let res = Cmpp3ConnRspPkt{
            status: 0,
            auth_ismg: "".to_string(),
            version: 0,
            secret: "".to_string(),
            auth_src: "".to_string(),
            seq_id: self.seq_id,
        };
        Ok(res)
    }

}

#[derive(Debug, Clone)]
pub struct Cmpp3ConnRspPkt {
    pub status: u32,
    pub auth_ismg: String,
    pub version: u8,
    pub secret: String,
    pub auth_src: String,
    pub seq_id: u32,
}

impl Cmpp3ConnRspPkt {
    pub fn pack(self) -> Result<Vec<u8>> {
        // pack header
        let mut buffer = Vec::with_capacity(CMPP3CONN_RSP_PKT_LEN as usize);
        buffer.put_u32(CMPP3CONN_RSP_PKT_LEN);
        buffer.put_u32(CMPP_CONNECT_RESP);
        buffer.put_u32(self.seq_id);

        // pack body
        // Status
        buffer.put_u32(self.status);

        // auth_msg
        let auth_src = octet_string(String::new(), 16);
        buffer.extend_from_slice(auth_src.as_bytes());
        // Version
        buffer.push(self.version);

        Ok(buffer)
    }

}