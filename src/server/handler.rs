use log::info;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::server::cmd::Command;

pub struct MsgInHandler {
    request_rx: Receiver<Command>, // 请求命令队列
    response_tx: Sender<Command>,  // 响应命令队列
}

impl MsgInHandler {
    pub fn new(rx: Receiver<Command>, tx: Sender<Command>) -> MsgInHandler {
        MsgInHandler {
            request_rx: rx,
            response_tx: tx,
        }
    }

    pub async fn run(&mut self) {
        self.handle_msg().await;
    }

    #[allow(unused_variables)]
    async fn handle_msg(&mut self) {
        let res_tx = self.response_tx.clone();

        // 处理请求消息
        while let Some(req) = self.request_rx.recv().await {
            info!("msg req: {:?}", req);

            match req {
                Command::Submit(ref submit) => {
                    // 投递响应
                    _ = res_tx.send(req.apply().unwrap()).await;

                    // 投递状态报告
                }
                _ => {}
            }
        }

        /*tokio::spawn(async move {
            let mut seq = 0;
            while let Some(req) = msg_rx.recv().await {
                info!("msg req: {:?}", req);
                seq += 1;

                let mut pkt = Cmpp3DeliverReqPkt::new();
                pkt.msg_id = req.msg_id;
                pkt.seq_id = seq;

                match res_tx.send(Command::DeliverReq(pkt)).await {
                    Ok(_) => {}
                    Err(e) => {
                        warn!("send deliver failed: {}", e);
                    }
                }
            }
        });


        let tx1 = self.response_tx.clone();
        while let Some(req) = self.request_rx.recv().await {

            match req {
                Command::Submit(ref submit) => {
                    // 发送到消息队列 处理消息 + 投递响应
                    _ = msg_tx.send(submit.clone()).await;
                    // 投递响应
                    _ = tx1.send(req.apply().unwrap()).await;
                }

                Command::Unknown(ref u) => {
                    warn!("known command_id {}", u.command_id);
                    continue
                }
                _ => {
                    let command = req.apply().unwrap();
                    let _ = tx1.send(command).await;
                }
            }

        }*/
    }
}