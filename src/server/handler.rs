use log::info;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::server::cmd::Command;

pub struct MsgInHandler {
    pub in_rx: Receiver<Command>,
    pub tx_out: Sender<Command>,
}

impl MsgInHandler {
    pub async fn run(&mut self)  {
        self.handle_msg().await;
    }

    async fn handle_msg(&mut self) {
        while let Some(req) = self.in_rx.recv().await {
            info!("IN = {:?}", req);

            match req {
                Command::Submit(ref submit) => {
                    // TODO
                }
                _ => {}
            }

            let command = req.apply().unwrap();
            let _ = self.tx_out.send(command).await;
        }
    }
}