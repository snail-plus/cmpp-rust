use log::{error, info};
use tokio::io::{AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::server::cmd::Command;
use crate::server::cmd::deliver::Cmpp3DeliverReqPkt;

pub struct MsgInHandler {
    pub(crate) writer: WriteHalf<TcpStream>,
    pub in_rx: Receiver<(Command, Command)>,
}

impl MsgInHandler {
    pub async fn run(&mut self)  {
        while let Some((req, res)) = self.in_rx.recv().await {
            info!("IN = {:?}", req);

            let _ = self.writer.write_all(&res.into_frame().unwrap()).await;
            let _ = self.writer.flush().await;

            match req {
                Command::Submit(submit) => {
                    // 投递状态报告
                    let mut pkt = Cmpp3DeliverReqPkt::new();
                    pkt.seq_id = submit.seq_id;
                    let msg_id = submit.msg_id;
                    pkt.msg_id = msg_id;

                    let pkt_res = Command::Deliver(pkt).into_frame().unwrap();
                    let _ = self.writer.write_all(&pkt_res).await;
                    let _ = self.writer.flush().await;
                }
                _ => {}
            }
        }
    }
}