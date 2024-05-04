use std::sync::{Arc, Mutex};
use std::time::Duration;
use bytes::BytesMut;

use log::{error, info};
use tokio::{io, time};
use tokio::io::{AsyncReadExt, AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::time::Interval;
use tokio_util::codec::Decoder;

use crate::server::{CmppDecoder};
use crate::server::cmd::Command;
use crate::server::cmd::deliver::Cmpp3DeliverReqPkt;
use crate::server::Result;


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);


pub struct Conn {
    interval: Arc<Mutex<Interval>>,
}

impl Conn {
    pub fn new() -> Conn {
        let i = time::interval(HEARTBEAT_INTERVAL);
        Conn {
            interval: Arc::new(Mutex::new(i)),
        }

    }

    pub async fn serve(&self, stream: TcpStream) -> Result<()> {
        let mut decoder = CmppDecoder::default();
        let mut buf = BytesMut::with_capacity(1024);
        let (mut rd, wr) = io::split(stream);

        let (out_tx, out_rx) = mpsc::channel::<Command>(256);
        let (in_tx, in_rx) = mpsc::channel::<Command>(256);

        let in_handler = MsgInHandler{ in_rx, out_tx: out_tx.clone()};
        tokio::spawn(async move {
            in_handler.run().await;
        });

        let out_handler = MsgOutHandler{ out_rx, wr};
        tokio::spawn(async move {
            out_handler.run().await;
        });

        info!("buf cap: {}", buf.capacity());

        loop {
            match rd.read_buf(&mut buf).await {
                Ok(read_size) => {
                    if read_size == 0 {
                       return Err("eof".into());
                    }

                    while let Some(mut frame) = decoder.decode(&mut buf)? {
                        self.interval.lock().unwrap().reset();
                        let in_tx = in_tx.clone();
                        let out_tx = out_tx.clone();
                        let command = Command::parse_frame(frame.command_id, &mut frame.body_data)?;
                        command.apply(in_tx, out_tx).await?
                    }
                }
                Err(e) => {
                    return Err(format!("{:?}", e).into());
                }
            }
        }
    }

}

struct MsgInHandler {
    in_rx: Receiver<Command>,
    out_tx: Sender<Command>
}

impl MsgInHandler {
    async fn run(mut self) {
        while let Some(cmd) = self.in_rx.recv().await {
            info!("IN = {:?}", cmd);
            match cmd {
                Command::Connect(_) => {}
                Command::Submit(_) => {
                    // 投递状态报告
                    let pkt = Cmpp3DeliverReqPkt::new();
                    let _ = self.out_tx.send(Command::Deliver(pkt)).await;
                }
                _ => {}
            }
        }
    }
}


struct MsgOutHandler {
    out_rx: Receiver<Command>,
    wr: WriteHalf<TcpStream>,
}

impl MsgOutHandler {
    async fn run(mut self) {
        while let Some(cmd) = self.out_rx.recv().await {
            info!("OUT = {:?}", cmd);
            match cmd.into_frame() {
                Ok(res) => {
                    if let Err(e) = self.wr.write_all(&res).await {
                        error!("write err: {:?}", e);
                        return;
                    }

                    if let Err(e) = self.wr.flush().await {
                        error!("flush err: {:?}", e);
                    }
                }
                Err(e) => {
                    error!("into frame err: {:?}", e)
                }
            }
        }
    }

}