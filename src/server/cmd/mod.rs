use crate::server::cmd::active::{CmppActiveTestReqPkt, CmppActiveTestRspPkt};
use crate::server::cmd::connect::{Cmpp3ConnRspPkt, CmppConnReqPkt};
use crate::server::cmd::deliver::Cmpp3DeliverReqPkt;
use crate::server::cmd::submit::{Cmpp3SubmitReqPkt, Cmpp3SubmitRspPkt};
use crate::server::cmd::unknown::Unknown;
use crate::server::Result;

pub mod connect;
mod unknown;
mod submit;
pub mod deliver;
pub mod active;

pub const CMPP_CONNECT: u32 = 1;
pub const CMPP_CONNECT_RESP: u32 = 2147483649;
pub const CMPP_SUBMIT: u32 = 4;
pub const CMPP_SUBMIT_RESP: u32 = 2147483652;

pub const CMPP_ACTIVE_TEST_REQ_PKT_LEN: u32 = 12;

pub const CMPP_ACTIVE_TEST: u32 = 8;
pub const CMPP_ACTIVE_TEST_RESP: u32 = 2147483656;

pub const CMPP_HEADER_LEN: u32 = 12;

//39d, 0x27
const CMPP3CONN_RSP_PKT_LEN: u32 = 4 + 4 + 4 + 4 + 16 + 1;    //33d, 0x21

const CMPP_DELIVER: u32 = 5;
const CMPP_DELIVER_RESP: u32 = 2147483653;


#[derive(Debug, Clone)]
pub enum Command {
    Connect(CmppConnReqPkt),
    ConnectRsp(Cmpp3ConnRspPkt),
    Submit(Cmpp3SubmitReqPkt),
    SubmitRsp(Cmpp3SubmitRspPkt),
    Deliver(Cmpp3DeliverReqPkt),
    ActiveTest(CmppActiveTestReqPkt),
    ActiveTestRsp(CmppActiveTestRspPkt),
    Unknown(Unknown),
}


impl  Command {
    pub fn parse_frame(command_id: u32, frame: &mut Vec<u8>) -> Result<Command> {
        let command = match command_id {
            CMPP_CONNECT => Command::Connect(CmppConnReqPkt::parse_frame(frame)?),
            CMPP_SUBMIT => Command::Submit(Cmpp3SubmitReqPkt::parse_frame(frame)?),
            CMPP_ACTIVE_TEST => Command::ActiveTest(CmppActiveTestReqPkt::parse_frame(frame)?),
            _ => {
                return Ok(Command::Unknown(Unknown::new(command_id)));
            }
        };

        Ok(command)
    }

    pub fn into_frame(self) -> Result<Vec<u8>> {
        match self {
            Command::ConnectRsp(res) => res.pack(),
            Command::SubmitRsp(res) => res.pack(),
            Command::Deliver(res) => res.pack(),
            _ => {Ok(vec![])}
        }
    }

    pub(crate) fn apply(&self) -> Result<Command> {
        match self {
            Command::Connect(ref cmd) => {
                cmd.apply().map(|t| { Command::ConnectRsp(t) })
            }
            Command::Submit(ref cmd) => {
                cmd.apply().map(|t| { Command::SubmitRsp(t) })
            }

            Command::ActiveTest(ref cmd) => {
                cmd.apply().map(|t| { Command::ActiveTestRsp(t) })
            }

            _ => Ok(Command::Unknown(Unknown::new(0)))
        }
    }

}