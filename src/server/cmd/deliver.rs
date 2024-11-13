use crate::server::Result;
use bytes::{Buf, BufMut};
use crate::server::cmd::{CMPP_DELIVER, CMPP_HEADER_LEN};
use crate::server::cmd::active::{CmppActiveTestReqPkt, CmppActiveTestRspPkt};
use crate::util::str::octet_string;

#[derive(Debug, Clone)]
pub struct Cmpp3DeliverReqPkt {
    pub msg_id: u64,
    pub dest_id: String,
    pub service_id: String,
    pub tp_pid: u8,
    pub tp_udhi: u8,
    pub msg_fmt: u8,
    pub src_terminal_id: String,
    pub src_terminal_type: u8,
    pub register_delivery: u8,
    pub msg_length: u8,
    pub msg_content: String,
    pub link_id: String,

    //session info
    pub seq_id: u32,
}

impl Cmpp3DeliverReqPkt {

    pub fn new() -> Cmpp3DeliverReqPkt {
        Cmpp3DeliverReqPkt{
            msg_id: 0,
            dest_id: "".to_string(),
            service_id: "".to_string(),
            tp_pid: 0,
            tp_udhi: 0,
            msg_fmt: 0,
            src_terminal_id: "".to_string(),
            src_terminal_type: 0,
            register_delivery: 0,
            msg_length: 0,
            msg_content: "".to_string(),
            link_id: "".to_string(),
            seq_id: 0,
        }
    }

    pub fn pack(&self) -> Result<Vec<u8>> {
        let pkt_len = CMPP_HEADER_LEN + 77 + self.msg_length as u32 + 20u32;
        let mut buffer = Vec::with_capacity(pkt_len as usize);

        buffer.put_u32(pkt_len);
        buffer.put_u32(CMPP_DELIVER);
        buffer.put_u32(self.seq_id);

        buffer.put_u64(self.msg_id);
        buffer.put_slice(octet_string(self.dest_id.clone(), 21).as_bytes());
        buffer.put_slice(octet_string(self.service_id.clone(), 10).as_bytes());
        buffer.put_u8(self.tp_pid);
        buffer.put_u8(self.tp_udhi);
        buffer.put_u8(self.msg_fmt);
        buffer.put_slice(octet_string(self.src_terminal_id.clone(), 32).as_bytes());
        buffer.put_u8(self.src_terminal_type);
        buffer.put_u8(self.register_delivery);
        buffer.put_u8(self.msg_length);
        buffer.put_slice(self.msg_content.clone().as_bytes());
        buffer.put_slice(octet_string(self.link_id.clone(), 20).as_bytes());

        Ok(buffer)
    }
}

#[derive(Debug, Clone)]
pub struct Cmpp3DeliverResPkt {
    pub msg_id: u64,
    pub result :u32,
    pub seq_id: u32,
}

impl Cmpp3DeliverResPkt {
    pub(crate) fn parse_frame(seq_id: u32, data: &Vec<u8>) -> Result<Cmpp3DeliverResPkt> {

        let mut buf = bytes::BytesMut::with_capacity(data.len());
        buf.extend_from_slice(data);

        let pkt = Cmpp3DeliverResPkt {
            msg_id: buf.get_u64(),
            result: buf.get_u32(),
            seq_id,
        };
        Ok(pkt)
    }
}