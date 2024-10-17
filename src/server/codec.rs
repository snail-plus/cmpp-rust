use std::io;
use std::io::{Read};

use tokio_util::bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

const CMPP3_PACKET_MAX: u32 = 3335;
const CMPP3_PACKET_MIN: u32 = 12;

const CMPP_HEADER_LEN: u32 = 12;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CmppMessage {
    total_length: u32,
    seq_id: u32,
    pub command_id: u32,
    pub body_data: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
enum DecodeState {
    Head(CmppHead),
    Data(usize),
}

#[derive(Debug, Clone, Copy)]
struct CmppHead {
    total_length: u32,
    command_id: u32,
    seq_id: u32,
}

#[derive(Clone, Copy)]
pub struct CmppDecoder {
    head: Option<CmppHead>,
}

impl CmppDecoder {
    pub fn new() -> Self {
        Self {
            head: None,
        }
    }
}

impl Default for CmppDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Decoder for CmppDecoder {
    type Item = CmppMessage;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<CmppMessage>, Self::Error> {
        
        if buf.len() < CMPP3_PACKET_MIN as usize {
            // Not enough data to read length marker.
            return Ok(None);
        }

        if self.head.is_none() {
            let total_length = buf.get_u32();
            let command_id = buf.get_u32();
            let seq_id = buf.get_u32();

            if total_length < CMPP3_PACKET_MIN || total_length > CMPP3_PACKET_MAX {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid length"));
            }

            self.head = Some(CmppHead{
                total_length,
                command_id,
                seq_id,
            });
        }

        let head = self.head.as_ref().unwrap();
        let total_length = head.total_length;
        let seq_id = head.seq_id;
        let command_id = head.command_id;

        let body_length = (head.total_length - CMPP_HEADER_LEN) as usize;
        if buf.remaining() < body_length {
            return Ok(None);
        }

        let mut body_buf = vec![0u8; body_length];
        buf.copy_to_slice(&mut body_buf);

        self.head = None;

        Ok(Some(CmppMessage{
            total_length,
            seq_id,
            command_id,
            body_data: body_buf,
        }))

    }
}


pub struct CmppEncoder;

impl Encoder<CmppMessage> for CmppEncoder {
    type Error = io::Error;

    fn encode(&mut self, item: CmppMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_u32(item.total_length);
        dst.put_u32(item.command_id);
        dst.extend_from_slice(&item.body_data);
        Ok(())
    }
}