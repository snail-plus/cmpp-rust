use log::info;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::server::cmd::Command;
use crate::server::cmd::deliver::Cmpp3DeliverReqPkt;

pub struct MsgInHandler {
    request_rx: Receiver<Command>, // 请求命令队列
    response_tx: Sender<Command>,  // 响应命令队列
}

impl MsgInHandler {
    pub fn new(rx: Receiver<Command>, tx: Sender<Command>) -> Self {
        Self {
            request_rx: rx,
            response_tx: tx,
        }
    }

    #[allow(unused_variables)]
    pub async fn run(&mut self) {
        let res_tx = self.response_tx.clone();
        // 处理请求消息
        while let Some(req) = self.request_rx.recv().await {
            info!("msg req: {:?}", req);

            match req {
                Command::Submit(ref submit) => {
                    // 投递响应
                    _ = res_tx.send(req.apply().unwrap()).await;
                    // 投递状态报告 待定
                    let mut report = Cmpp3DeliverReqPkt::new();
                    report.msg_id = submit.msg_id;
                    report.seq_id = submit.seq_id;
                    report.dest_id = submit.dest_terminal_id[0].clone();
                    _ = res_tx.send(Command::DeliverReq(report)).await;
                }
                _ => {}
            }
        }
    }

}