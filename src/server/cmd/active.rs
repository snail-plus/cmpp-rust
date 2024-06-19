use bytes::{Buf, BufMut};

use crate::server::cmd::CMPP_ACTIVE_TEST;

#[derive(Debug, Clone)]
pub struct CmppActiveTestReqPkt {
    pub(crate) seq_id: u32,
}

impl CmppActiveTestReqPkt {

    pub(crate) fn parse_frame(data: &mut Vec<u8>) -> crate::server::Result<CmppActiveTestReqPkt> {
        let mut buf = bytes::BytesMut::with_capacity(data.len());
        let mut pkt = CmppActiveTestReqPkt{seq_id: 0};
        let seq_id = buf.get_u32();
        pkt.seq_id = seq_id;
        Ok(pkt)
    }

    pub fn pack(&self, seq_id: u32) -> crate::server::Result<Vec<u8>> {
        let pkt_len = CMPP_ACTIVE_TEST + 4;
        let mut buffer = Vec::with_capacity(pkt_len as usize);
        buffer.put_u32(pkt_len);
        buffer.put_u32(CMPP_ACTIVE_TEST);
        buffer.put_u32(seq_id);
        Ok(buffer)
    }

    pub(crate) fn apply(&self) -> crate::server::Result<CmppActiveTestRspPkt> {
        let res = CmppActiveTestRspPkt{
            reserved: 0,
            seq_id: self.seq_id,
        };
        Ok(res)
    }

}

#[derive(Debug, Clone)]
pub struct CmppActiveTestRspPkt {
    reserved: u8,
    // session info
    seq_id: u32
}

