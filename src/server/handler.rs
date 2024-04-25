use std::io;
use std::sync::Mutex;
use log::info;
use crate::server::packet::Packet;
use crate::server::response::Response;


pub trait CmppHandler: Send + Sync {
    fn handle(&self, r: &mut Response) -> io::Result<bool>;
}


pub struct LoginHandler;

impl CmppHandler for LoginHandler {
    fn handle(&self, _r: &mut Response) -> io::Result<bool> {
        info!("please handle msg");
        Ok(true)
    }
}