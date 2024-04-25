use std::io;
use std::sync::Mutex;
use log::info;
use crate::server::packet::{Cmpp3ConnRspPkt, Cmpp3SubmitReqPkt, Cmpp3SubmitRspPkt, CMPP_CONNECT, CMPP_SUBMIT, CmppConnReqPkt, Packer, Packet};


pub trait CmppHandler<T: Packer, R: Packer>: Send + Sync {
    fn handle(&self, req: &T, res: &mut T) -> io::Result<bool>;
}


pub struct CmppLoginHandler;

impl CmppHandler<CmppConnReqPkt, Cmpp3ConnRspPkt> for CmppLoginHandler {
    fn handle(&self, req: &CmppConnReqPkt, res: &mut Cmpp3ConnRspPkt) -> io::Result<bool> {
        info!("please handle CMPP_CONNECT msg, {:?}", req);
        Ok(true)
    }
}

pub struct Cmpp3SubmitHandler;
impl CmppHandler<Cmpp3SubmitReqPkt, Cmpp3SubmitRspPkt> for Cmpp3SubmitHandler {
    fn handle(&self, req: &Cmpp3SubmitReqPkt, res: &mut Cmpp3SubmitRspPkt) -> io::Result<bool> {
        info!("please handle CMPP_SUBMIT msg, {:?}", req);
        Ok(true)
    }
}