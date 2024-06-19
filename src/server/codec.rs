use std::io;
use std::io::Cursor;

use tokio_util::bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

const CMPP3_PACKET_MAX: u32 = 3335;
const CMPP3_PACKET_MIN: u32 = 12;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CmppMessage {
    total_length: u32,
    pub command_id: u32,
    pub body_data: Vec<u8>,
}

#[derive(Clone, Copy)]
pub struct CmppDecoder;

impl CmppDecoder {
    pub fn new() -> CmppDecoder {
        CmppDecoder {}
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

        if buf.len() < 8usize {
            // Not enough data to read length marker.
            return Ok(None);
        }

        let mut cursor = Cursor::new(&buf[..]);
        let pos = cursor.position();
        let total_length = cursor.get_u32();
        let command_id = cursor.get_u32();

        if total_length < CMPP3_PACKET_MIN || total_length > CMPP3_PACKET_MAX {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid length"));
        }

        let body_length = (total_length - 8) as usize;

        if cursor.remaining() < body_length {
            // The full data has not yet arrived.
            //
            // We reserve more space in the buffer. This is not strictly
            // necessary, but is a good idea performance-wise.
            cursor.set_position(pos);
            // reset position
            // We inform the Framed that we need more bytes to form the next
            // frame.
            return Ok(None);
        }

        let mut body_buf = vec![0u8; body_length];
        cursor.copy_to_slice(&mut body_buf);
        buf.advance(8 + body_length);

        Ok(Some(CmppMessage{
            total_length,
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