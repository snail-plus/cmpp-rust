use std::time::{Duration};

use bytes::BytesMut;
use log::{error, info};
use tokio::{io};
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_util::codec::Decoder;

use crate::server::cmd::Command;
use crate::server::cmd::deliver::Cmpp3DeliverReqPkt;
use crate::server::CmppDecoder;
use crate::server::handler::MsgInHandler;
use crate::server::Result;


pub struct Conn {
    reader: ReadHalf<TcpStream>,
    writer: WriteHalf<TcpStream>,
    buf: BytesMut,
    rx: Receiver<Command>,
    tx: Sender<Command>,
}

impl Conn {
    pub fn new(stream: TcpStream) -> Conn {
        let (mut reader, mut writer) = io::split(stream);
        let mut buf = BytesMut::with_capacity(1024);
        let (tx_in, rx_in) = tokio::sync::mpsc::channel(1024);
        let (tx_out, rx_out) = tokio::sync::mpsc::channel(1024);

        let mut handler = MsgInHandler {
            in_rx: rx_in,
            out_tx: tx_out,
        };
        tokio::spawn(async move {
            handler.run().await;
        });

        Conn { reader, writer, buf, rx: rx_out, tx: tx_in }
    }

    pub async fn serve(&mut self) -> Result<()> {
        self.read_frame().await?;
        // 读取队列中的状态报告投递给客户端
        self.deliver().await
    }

    async fn deliver(&mut self) -> Result<()> {
        while let Some(cmd) = self.rx.recv().await {
            info!("OUT={:?}", cmd);
            let res = cmd.into_frame()?;
            self.writer.write_all(&res).await?;
            self.writer.flush().await?;
        }
        Ok(())
    }

    async fn read_frame(&mut self) -> Result<()> {
        let mut decoder = CmppDecoder::default();

        let buf = &mut self.buf;
        loop {
            match self.reader.read_buf(buf).await {
                Ok(read_size) => {
                    if read_size == 0 {
                        return if buf.is_empty() {
                            Ok(())
                        } else {
                            Err("connection reset by peer".into())
                        };
                    }

                    while let Some(mut frame) = decoder.decode(buf)? {
                        let mut command = Command::parse_frame(frame.command_id, &mut frame.body_data)?;
                        command.apply(&mut self.writer).await?;
                        let sender = self.tx.clone();
                        sender.send(command).await?;
                    }
                }
                Err(e) => {
                    return Err(format!("{:?}", e).into());
                }
            }
        }
    }
}

