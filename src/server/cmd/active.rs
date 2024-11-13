use bytes::BufMut;

use crate::server::cmd::CMPP_ACTIVE_TEST;

#[derive(Debug, Clone)]
pub struct CmppActiveTestReqPkt {
    pub(crate) seq_id: u32,
}

impl CmppActiveTestReqPkt {

    pub(crate) fn parse_frame(seq_id : u32) -> crate::server::Result<CmppActiveTestReqPkt> {
        let pkt = CmppActiveTestReqPkt{seq_id };
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

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct CmppActiveTestRspPkt {
    reserved: u8,
    // session info
    seq_id: u32
}

