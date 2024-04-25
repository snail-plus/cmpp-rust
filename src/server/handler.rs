use std::io;
use std::sync::Mutex;
use log::info;
use crate::server::packet::{CMPP_CONNECT, CMPP_SUBMIT, Packet};


pub trait CmppHandler: Send + Sync {
    fn handle(&self, req: &Packet, res: &mut Packet) -> io::Result<bool>;
}


pub struct CmppLoginHandler;

impl CmppHandler for CmppLoginHandler {
    fn handle(&self, req: &Packet, res: &mut Packet) -> io::Result<bool> {
        if req.command_id != CMPP_CONNECT {
            return Ok(false)
        }

        info!("please handle CMPP_CONNECT msg, {:?}", req);
        Ok(true)
    }
}

pub struct Cmpp3SubmitHandler;
impl CmppHandler for Cmpp3SubmitHandler {
    fn handle(&self, req: &Packet, res: &mut Packet) -> io::Result<bool> {
        if req.command_id != CMPP_SUBMIT {
            return Ok(false)
        }
        info!("please handle CMPP_SUBMIT msg, {:?}", req);
        Ok(true)
    }
}