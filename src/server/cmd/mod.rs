use tokio::io::{AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;
use crate::server::cmd::connect::CmppConnReqPkt;
use crate::server::cmd::submit::{Cmpp3SubmitReqPkt, Cmpp3SubmitRspPkt};
use crate::server::cmd::unknown::Unknown;
use crate::server::Result;

mod connect;
mod unknown;
mod submit;
mod deliver;
pub mod active;

pub const CMPP_CONNECT: u32 = 1;
pub const CMPP_CONNECT_RESP: u32 = 2147483649;
pub const CMPP_SUBMIT: u32 = 4;
pub const CMPP_SUBMIT_RESP: u32 = 2147483652;

pub const CMPP_ACTIVE_TEST_REQ_PKT_LEN: u32 = 12;

pub const CMPP_ACTIVE_TEST: u32 = 8;
pub const CMPP_ACTIVE_TEST_RESP: u32 = 2147483656;

pub const CMPP_HEADER_LEN: u32 = 12;

const CMPP_CONN_REQ_PKT_LEN: u32 = 4 + 4 + 4 + 6 + 16 + 1 + 4;
//39d, 0x27
const CMPP3CONN_RSP_PKT_LEN: u32 = 4 + 4 + 4 + 4 + 16 + 1;    //33d, 0x21

const CMPP_DELIVER: u32 = 5;
const CMPP_DELIVER_RESP: u32 = 2147483653;


#[derive(Debug, Clone)]
pub enum Command {
    Connect(CmppConnReqPkt),
    Submit(Cmpp3SubmitReqPkt),
    Unknown(Unknown),
}


impl Command {
    pub fn parse_frame(command_id: u32, frame: &mut Vec<u8>) -> Result<Command> {
        let command = match command_id {
            CMPP_CONNECT => Command::Connect(CmppConnReqPkt::parse_frame(frame)?),
            CMPP_SUBMIT => Command::Submit(Cmpp3SubmitReqPkt::parse_frame(frame)?),
            _ => {
                return Ok(Command::Unknown(Unknown::new(command_id)));
            }
        };

        Ok(command)
    }

    pub(crate) async fn apply(self, tx: Sender<Command>, wh: &mut WriteHalf<TcpStream>) -> Result<()> {
        match self {
            Command::Connect(ref cmd) => {
                cmd.apply(wh).await?;
            }
            Command::Submit(ref cmd) => {
                cmd.apply(wh).await?;
                tx.send(self).await?
            }
            Command::Unknown(cmd) => {}
        }
        Ok(())
    }

}