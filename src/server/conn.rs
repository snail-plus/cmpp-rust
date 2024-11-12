use bytes::BytesMut;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf};
use tokio::net::TcpStream;
use tokio_util::codec::Decoder;

use crate::server::{cmd, CmppDecoder};
use crate::server::cmd::Command;
use crate::server::handler::MsgInHandler;
use crate::server::Result;

pub trait AuthHandler: Send + Sync {
    fn auth(&self, req: &cmd::connect::CmppConnReqPkt, res: &mut cmd::connect::Cmpp3ConnRspPkt) -> bool;
}

pub struct DefaultAuthHandler {}

impl AuthHandler for DefaultAuthHandler {
    fn auth(&self, _req: &cmd::connect::CmppConnReqPkt, res: &mut cmd::connect::Cmpp3ConnRspPkt) -> bool {
        res.status = 0;
        res.auth_ismg = "认证成功".to_string();
        true
    }
}

pub struct Conn {
    buf: BytesMut,
    auth_handler: Box<dyn AuthHandler>,
}

impl Conn {
    pub fn new() -> Conn {
        let buf = BytesMut::with_capacity(2048);
        Conn { buf, auth_handler: Box::new(DefaultAuthHandler {}) }
    }

    pub async fn run(&mut self, stream: TcpStream) -> Result<()> {
        let (mut reader, mut writer) = io::split(stream);

        let (tx_in, rx_in) = tokio::sync::mpsc::channel(1024);
        let (tx_out, mut rx_out) = tokio::sync::mpsc::channel(1024);

        // 根据客户端IP 创建限流
        let mut handler = MsgInHandler::new(rx_in, tx_out.clone());
        tokio::spawn(async move {
            handler.run().await;
        });

        tokio::spawn(async move {
            while let Some(req) = rx_out.recv().await {
                let _ = writer.write_all(&req.into_frame().unwrap()).await;
                let _ = writer.flush().await;
            }
        });

        loop {
            if let Some(req) = self.read_frame(&mut reader).await? {
                match req {
                    Command::Connect(ref req_c) => {
                        let mut res = req.apply()?;
                        match res {
                            Command::ConnectRsp(ref mut res_c) => {
                                let auth_result = self.auth_handler.auth(&req_c, res_c);
                                tx_out.clone().send(res).await?;
                                if !auth_result {
                                    return Err("认证失败".into());
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        let sender = tx_in.clone();
                        sender.send(req).await?;
                    }
                }
            }
        }
    }


    async fn read_frame(&mut self, reader: &mut ReadHalf<TcpStream>) -> Result<Option<Command>> {
        let mut decoder = CmppDecoder::default();

        loop {
            if let Some(mut frame) = decoder.decode(&mut self.buf)? {
                let req = Command::parse_frame(frame.command_id, frame.seq_id, &mut frame.body_data)?;
                return Ok(Some(req));
            }

            if 0 == reader.read_buf(&mut self.buf).await? {
                return if self.buf.is_empty() {
                    Ok(None)
                } else {
                    let s = "connection reset by peer".into();
                    Err(s)
                };
            }
        }
    }
}

