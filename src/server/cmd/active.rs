
#[derive(Debug, Clone)]
pub struct CmppActiveTestReqPkt {
    pub(crate) seq_id: u32,
}

impl CmppActiveTestReqPkt {
    pub fn pack(&self, seq_id: u32) -> crate::server::Result<Vec<u8>> {
        Ok(vec![])
    }

}