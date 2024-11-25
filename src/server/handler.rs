use log::info;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::server::cmd::Command;

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
                }
                _ => {}
            }
        }
    }

}