use std::io;
use std::sync::Mutex;
use log::info;
use crate::server::IoError;
use crate::server::packet::{CMPP_CONNECT, CMPP_SUBMIT, Packet};


pub trait CmppHandler: Send + Sync {
    fn handle(&self, req: &Packet, res: &mut Packet) -> Result<(), IoError>;

    fn support(&self, command_id: u32) -> bool;
}


pub struct CmppLoginHandler;

impl CmppHandler for CmppLoginHandler {
    fn handle(&self, req: &Packet, res: &mut Packet) -> Result<(), IoError> {
        info!("please handle CMPP_CONNECT msg, {:?}", req);
        Ok(())
    }

    fn support(&self, command_id: u32) -> bool {
        command_id == CMPP_CONNECT
    }
}

pub struct Cmpp3SubmitHandler;
impl CmppHandler for Cmpp3SubmitHandler {
    fn handle(&self, req: &Packet, res: &mut Packet) -> Result<(), IoError> {
        info!("please handle CMPP_SUBMIT msg, {:?}", req);
        Ok(())
    }

    fn support(&self, command_id: u32) -> bool {
        command_id == CMPP_SUBMIT
    }

}