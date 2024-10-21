use crate::server::cmd::active::{CmppActiveTestReqPkt, CmppActiveTestRspPkt};
use crate::server::cmd::connect::{Cmpp3ConnRspPkt, CmppConnReqPkt};
use crate::server::cmd::deliver::Cmpp3DeliverReqPkt;
use crate::server::cmd::submit::{Cmpp3SubmitReqPkt, Cmpp3SubmitRspPkt};
use crate::server::cmd::unknown::Unknown;
use crate::server::Result;

pub mod connect;
mod unknown;
pub(crate) mod submit;
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


// 连接失败枚举
pub const ERRNO_CONN_INVALID: u8 = 1;
pub const ERRNO_CONN_INVALID_SRC_ADDR: u8 = 2;
pub const ERRNO_CONN_AUTH_FAILED: u8 = 3;
pub const ERRNO_CONN_VER_TOO_HIGH: u8 = 4;
pub const ERRNO_CONN_OTHERS: u8 = 5;


#[derive(Debug, Clone)]
pub enum Command {
    Connect(CmppConnReqPkt),
    ConnectRsp(Cmpp3ConnRspPkt),
    Submit(Cmpp3SubmitReqPkt),
    SubmitRsp(Cmpp3SubmitRspPkt),
    ActiveTest(CmppActiveTestReqPkt),
    ActiveTestRsp(CmppActiveTestRspPkt),
    DeliverReq(Cmpp3DeliverReqPkt),
    Unknown(Unknown),
}


impl  Command {
    pub fn parse_frame(command_id: u32, seq_id: u32, frame: &mut Vec<u8>) -> Result<Command> {
        let command = match command_id {
            CMPP_CONNECT => Command::Connect(CmppConnReqPkt::parse_frame(seq_id, frame)?),
            CMPP_SUBMIT => Command::Submit(Cmpp3SubmitReqPkt::parse_frame(seq_id, frame)?),
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
            Command::DeliverReq(res) => res.pack(),
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