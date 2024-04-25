use crate::server::packet::Packer;

pub struct Response {
    pub packer: Box<dyn Packer>,
    pub seq_id: u32
}