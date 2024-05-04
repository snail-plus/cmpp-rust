use bytes::BufMut;
use tokio::sync::mpsc::Sender;

use crate::server::cmd::{CMPP3CONN_RSP_PKT_LEN, CMPP_CONNECT_RESP, Command};
use crate::server::Result;
use crate::util::str::octet_string;

#[derive(Debug, Clone)]
pub struct CmppConnReqPkt {
    pub src_addr: String,
    pub auth_src: String,
    pub version: String,
    pub timestamp: u32,
    pub secret: String,
    pub seq_id: u32,
}

impl CmppConnReqPkt {

    fn new() -> CmppConnReqPkt {
        CmppConnReqPkt {
            src_addr: "".to_string(),
            auth_src: "".to_string(),
            version: "".to_string(),
            timestamp: 0,
            secret: "".to_string(),
            seq_id: 0,
        }
    }

    pub(crate) fn parse_frame(_data: &mut Vec<u8>) -> Result<CmppConnReqPkt>{
        let pkt = CmppConnReqPkt::new();
        Ok(pkt)
    }

    pub(crate) async fn apply(&self, tx_out: Sender<Command>) {
        let res = Cmpp3ConnRspPkt{
            status: 0,
            auth_ismg: "".to_string(),
            version: 0,
            secret: "".to_string(),
            auth_src: "".to_string(),
            seq_id: self.seq_id,
        };
        let _ = tx_out.send(Command::ConnectRsp(res));
    }

}

#[derive(Debug, Clone)]
pub struct Cmpp3ConnRspPkt {
    status: u32,
    auth_ismg: String,
    version: u8,
    secret: String,
    auth_src: String,
    seq_id: u32,
}

impl Cmpp3ConnRspPkt {
    fn pack(self) -> Result<Vec<u8>> {
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