use bytes::BytesMut;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf};
use tokio::net::TcpStream;
use tokio_util::codec::Decoder;

use crate::server::{cmd, CmppDecoder};
use crate::server::cmd::Command;
use crate::server::handler::MsgInHandler;
use crate::server::Result;

pub trait  AuthHandler : Send + Sync{
    fn  auth(&self, req: &cmd::connect::CmppConnReqPkt, res: &mut cmd::connect::Cmpp3ConnRspPkt) -> bool;
}

pub struct DefaultAUthHandler {}

impl AuthHandler for DefaultAUthHandler {
    fn auth(&self, _req: &cmd::connect::CmppConnReqPkt, _res: &mut cmd::connect::Cmpp3ConnRspPkt) -> bool {
        true
    }
}

pub struct Conn {
    buf: BytesMut,
    auth_handler:  Box<dyn AuthHandler>,
}

impl Conn {
    pub fn new() -> Conn {
        let buf = BytesMut::with_capacity(1024);
        Conn {buf, auth_handler: Box::new(DefaultAUthHandler{}) }
    }

    pub async fn run(&mut self, stream: TcpStream) -> Result<()> {
        let (mut reader, mut writer) = io::split(stream);
        
        let (tx_in, rx_in) = tokio::sync::mpsc::channel(1024);
        let (tx_out, mut rx_out) = tokio::sync::mpsc::channel(1024);

        let tx_out_sender1 = tx_out.clone();
        let mut handler = MsgInHandler {
            in_rx: rx_in,
            tx_out: tx_out_sender1,
        };
        tokio::spawn(async move {
            handler.run().await;
        });

        tokio::spawn(async move {
            while let Some(mut req) = rx_out.recv().await {
                let _ = writer.write_all(&req.into_frame().unwrap()).await;
                let _ = writer.flush().await;
            }
        });

        loop {

             match self.read_frame(&mut reader).await? {
                 Some(req) => {
                     match req {
                         Command::Connect(ref req_c) => {
                             let mut res = req.apply()?;
                             match res {
                                 Command::ConnectRsp(ref mut res_c) => {
                                     let auth_result = self.auth_handler.auth(&req_c, res_c);
                                     let tx_out_sender2 = tx_out.clone();
                                     tx_out_sender2.send(res).await.unwrap();
                                     if !auth_result {
                                         return Err("认证失败".into())
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

                 None => {}
             }
        }
    }

    async fn read_frame(&mut self, reader: &mut ReadHalf<TcpStream>) -> Result<Option<Command>> {
        let mut decoder = CmppDecoder::default();

        let buf = &mut self.buf;
        loop {
            match reader.read_buf(buf).await {
                Ok(read_size) => {
                    if read_size == 0 {
                        return if buf.is_empty() {
                            Ok(None)
                        } else {
                            Err("connection reset by peer".into())
                        };
                    }

                    while let Some(mut frame) = decoder.decode(buf)? {
                        let req = Command::parse_frame(frame.command_id, &mut frame.body_data)?;
                        return Ok(Some(req))
                    }
                }
                Err(e) => {
                    return Err(format!("{:?}", e).into());
                }
            }
        }
    }
}

