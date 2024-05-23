use log::{error, info};
use tokio::sync::mpsc::{Receiver, Sender};
use crate::server::cmd::Command;
use crate::server::cmd::deliver::Cmpp3DeliverReqPkt;

pub struct MsgInHandler {
    pub in_rx: Receiver<Command>,
    pub out_tx: Sender<Command>,
}

impl MsgInHandler {
    pub async fn run(&mut self) {
        while let Some(cmd) = self.in_rx.recv().await {
            info!("IN = {:?}", cmd);
            match cmd {
                Command::Connect(_) => {}
                Command::Submit(submit) => {
                    // 投递状态报告
                    let mut pkt = Cmpp3DeliverReqPkt::new();
                    pkt.seq_id = submit.seq_id;
                    let msg_id = submit.msg_id;
                    pkt.msg_id = msg_id;
                    let sender = self.out_tx.clone();
                    if let Err(e) = sender.send(Command::Deliver(pkt)).await {
                        error!("send deliver err: {:?}", e)
                    }else {
                        info!("send deliver ok, msg_id: {}", msg_id)
                    }

                }
                _ => {}
            }
        }
    }
}