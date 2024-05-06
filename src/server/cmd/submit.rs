use bytes::{Buf, BufMut};
use log::error;
use tokio::io::{AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;

use crate::server::cmd::{CMPP_HEADER_LEN, CMPP_SUBMIT_RESP};
use crate::server::Result;
use crate::util::str::{oct_string, ucs2_to_utf8};

#[derive(Debug, Clone)]
pub struct Cmpp3SubmitReqPkt {
    pub msg_id: u64,
    pub pk_total: u8,
    pub pk_number: u8,
    pub registered_delivery: u8,
    pub msg_level: u8,
    pub service_id: String,
    pub fee_user_type: u8,
    pub fee_terminal_id: String,
    pub fee_terminal_type: u8,
    pub tp_pid: u8,
    pub tp_udhi: u8,
    pub msg_fmt: u8,
    pub msg_src: String,
    pub fee_type: String,
    pub fee_code: String,
    pub valid_time: String,
    pub at_time: String,
    pub src_id: String,
    pub dest_usr_tl: u8,
    pub dest_terminal_id: Vec<String>,
    pub dest_terminal_type: u8,
    msg_length: u8,
    pub msg_content: String,
    pub link_id: String,

    // session info
    pub seq_id: u32,
}


impl Cmpp3SubmitReqPkt {

    fn new() -> Cmpp3SubmitReqPkt {
        Cmpp3SubmitReqPkt {
            msg_id: 1,
            pk_total: 0,
            pk_number: 0,
            registered_delivery: 0,
            msg_level: 0,
            service_id: "".to_string(),
            fee_user_type: 0,
            fee_terminal_id: "".to_string(),
            fee_terminal_type: 0,
            tp_pid: 0,
            tp_udhi: 0,
            msg_fmt: 0,
            msg_src: "".to_string(),
            fee_type: "".to_string(),
            fee_code: "".to_string(),
            valid_time: "".to_string(),
            at_time: "".to_string(),
            src_id: "".to_string(),
            dest_usr_tl: 0,
            dest_terminal_id: vec![],
            dest_terminal_type: 0,
            msg_length: 0,
            msg_content: "".to_string(),
            link_id: "".to_string(),
            seq_id: 0,
        }
    }

    pub(crate) fn parse_frame(data: &mut Vec<u8>) -> Result<Cmpp3SubmitReqPkt> {
        let mut pkt = Cmpp3SubmitReqPkt::new();
        let mut buf = bytes::BytesMut::with_capacity(data.len());
        buf.extend_from_slice(data);
        // Sequence Id
        pkt.seq_id = buf.get_u32();
        // msg_id
        pkt.msg_id = buf.get_u64();
        pkt.pk_total = buf.get_u8();
        pkt.pk_number = buf.get_u8();

        pkt.registered_delivery = buf.get_u8();
        pkt.msg_level = buf.get_u8();

        let mut service_id_vec = vec![0u8; 10];
        buf.copy_to_slice(&mut service_id_vec);
        pkt.service_id = oct_string(service_id_vec);

        pkt.fee_user_type = buf.get_u8();

        let mut fee_terminal_id_vec = vec![0u8; 32];
        buf.copy_to_slice(&mut fee_terminal_id_vec);
        pkt.fee_terminal_id = oct_string(fee_terminal_id_vec);

        pkt.fee_terminal_type = buf.get_u8();

        pkt.tp_pid = buf.get_u8();
        pkt.tp_udhi = buf.get_u8();
        pkt.msg_fmt = buf.get_u8();

        let mut msg_src_vec = vec![0u8; 6];
        buf.copy_to_slice(&mut msg_src_vec);
        pkt.msg_src = oct_string(msg_src_vec);

        let mut fee_type_vec = vec![0u8; 2];
        buf.copy_to_slice(&mut fee_type_vec);
        pkt.fee_type = oct_string(fee_type_vec);

        let mut fee_code_vec = vec![0u8; 6];
        buf.copy_to_slice(&mut fee_code_vec);
        pkt.fee_code = oct_string(fee_code_vec);

        let mut valid_time_vec = vec![0u8; 17];
        buf.copy_to_slice(&mut valid_time_vec);
        pkt.valid_time = oct_string(valid_time_vec);

        let mut at_time_vec = vec![0u8; 17];
        buf.copy_to_slice(&mut at_time_vec);
        pkt.at_time = oct_string(at_time_vec);

        let mut src_id_vec = vec![0u8; 21];
        buf.copy_to_slice(&mut src_id_vec);
        pkt.src_id = oct_string(src_id_vec);

        pkt.dest_usr_tl = buf.get_u8();
        let mut dest_terminal_ids = Vec::with_capacity(pkt.dest_usr_tl as usize);
        for _i in 0..pkt.dest_usr_tl {
            let mut dest_terminal_id_vec = vec![0u8; 32];
            buf.copy_to_slice(&mut dest_terminal_id_vec);
            dest_terminal_ids.push(oct_string(dest_terminal_id_vec));
        }
        pkt.dest_terminal_id = dest_terminal_ids;

        pkt.dest_terminal_type = buf.get_u8();

        pkt.msg_length = buf.get_u8();
        let mut msg_content_vec = vec![0u8; pkt.msg_length as usize];
        buf.copy_to_slice(&mut msg_content_vec);
        match ucs2_to_utf8(msg_content_vec.as_slice()) {
            Ok(content) => { pkt.msg_content = content }
            Err(_e) => { return Err("解析msg_content失败".into())}
        }

        let mut link_id_vec = vec![0u8; 20];
        buf.copy_to_slice(&mut link_id_vec);
        pkt.link_id = oct_string(link_id_vec);
        Ok(pkt)
    }

    pub(crate) async fn apply(&self, w: &mut WriteHalf<TcpStream>) {
        let res = Cmpp3SubmitRspPkt{
            msg_id: self.msg_id,
            result: 0,
            seq_id: self.seq_id,
        };

        if let Err(e) = w.write_all(&res.pack().unwrap()).await  {
            error!("send submit rsp err: {:?}", e);
            return;
        }

        if let Err(e) = w.flush().await {
            error!("flush err: {:?}", e);
        }
    }

}


#[derive(Debug, Clone)]
pub struct Cmpp3SubmitRspPkt {
    pub(crate) msg_id: u64,
    pub(crate) result: u32,
    // session info
    pub(crate) seq_id: u32,
}

impl  Cmpp3SubmitRspPkt {

    pub(crate) fn pack(self) -> Result<Vec<u8>> {
        let pkt_len = CMPP_HEADER_LEN + 8 + 4;
        let mut buffer = Vec::with_capacity(pkt_len as usize);
        // Pack header

        buffer.put_u32(pkt_len);
        buffer.put_u32(CMPP_SUBMIT_RESP);
        buffer.put_u32(self.seq_id);

        // Pack Body
        buffer.put_u64(self.msg_id);
        buffer.put_u32(self.result);
        Ok(buffer)
    }


}