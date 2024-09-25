use std::sync::Arc;
use log::{info, warn};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{Semaphore, TryAcquireError};

use crate::server::cmd::Command;

pub struct MsgInHandler {
    rx: Receiver<Command>,
    tx: Sender<Command>,
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
            tx,
            limit_msg,
        }
    }

    pub async fn run(&mut self)  {
        self.handle_msg().await;
    }

    async fn handle_msg(&mut self) {
        while let Some(req) = self.rx.recv().await {
            info!("IN = {:?}", req);

            match req {
                Command::Submit(ref submit) => {
                    match self.limit_msg.try_acquire().err() {
                        Some(e) => {
                            warn!("limit msg: {}", e);
                            let _= self.tx.send(submit.apply().map(|mut i| {
                                i.result = 4;
                                Command::SubmitRsp(i)
                            }).unwrap()).await;
                            continue
                        }
                        _ => {
                            let _ = self.tx.send(req.apply().unwrap()).await;
                        }
                    }

                }

                Command::Unknown(ref u) => {
                    warn!("known command_id {}", u.command_id);
                    continue
                }
                _ => {}
            }

            let command = req.apply().unwrap();
            let _ = self.tx.send(command).await;
        }
    }
}