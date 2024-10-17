use std::sync::Arc;
use log::{info, warn};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{Semaphore};

use crate::server::cmd::Command;
use crate::server::cmd::deliver::Cmpp3DeliverReqPkt;
use crate::server::cmd::submit::Cmpp3SubmitReqPkt;

pub struct MsgInHandler {
    rx: Receiver<Command>,
    response_tx: Sender<Command>,
    limit_msg: Arc<Semaphore>
}

impl MsgInHandler {

    fn drop(&mut self) {
        // Add a permit back to the semaphore.
        //
        // Doing so unblocks the listener if the max number of
        // connections has been reached.
        //
        // This is done in a `Drop` implementation in order to guarantee that
        // the permit is added even if the task handling the connection panics.
        // If `add_permit` was called at the end of the `run` function and some
        // bug causes a panic. The permit would never be returned to the
        // semaphore.
        self.limit_msg.add_permits(1);
    }

    pub fn new(rx: Receiver<Command>, tx: Sender<Command>, limit_msg: Arc<Semaphore>) -> MsgInHandler {

        MsgInHandler{
            rx,
            response_tx: tx,
            limit_msg,
        }
    }

    pub async fn run(&mut self)  {
        self.handle_msg().await;
    }

    async fn handle_msg(&mut self) {

        let (msg_tx, mut msg_rx) = tokio::sync::mpsc::channel::<Cmpp3SubmitReqPkt>(10240);

        let res_tx = self.response_tx.clone();
        tokio::spawn(async move {
            while let Some(req) = msg_rx.recv().await {
                info!("msg req: {:?}", req);

                let mut seq = 0;
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
        while let Some(req) = self.rx.recv().await {

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

        }
    }
}