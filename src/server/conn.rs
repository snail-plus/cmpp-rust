use bytes::BytesMut;
use tokio::io;
use tokio::io::{AsyncReadExt, ReadHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;
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
    reader: ReadHalf<TcpStream>,
    buf: BytesMut,
    tx: Sender<(Command, Command)>,
    auth_handler:  Box<dyn AuthHandler>,
}

impl Conn {
    pub fn new(stream: TcpStream) -> Conn {
        let (reader, writer) = io::split(stream);
        let buf = BytesMut::with_capacity(1024);
        let (tx_in, rx_in) = tokio::sync::mpsc::channel(1024);

        let mut handler = MsgInHandler {
            writer,
            in_rx: rx_in,
        };
        tokio::spawn(async move {
            handler.run().await;
        });

        Conn { reader, buf, tx: tx_in, auth_handler: Box::new(DefaultAUthHandler{})}
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
             if let Some(mut req) = self.read_frame().await? {
                 let mut res = req.apply()?;
                 let sender = self.tx.clone();
                 match req {
                     Command::Connect(ref req_c) => {
                         match res {
                             Command::ConnectRsp(ref mut res_c) => {
                                 if !self.auth_handler.auth(&req_c, res_c) {
                                     return Err("认证失败".into())
                                 }

                                 sender.send((req, res)).await?;
                             }
                             _ => {}
                         }
                     }
                     _ => {
                         sender.send((req, res)).await?;
                     }
                 }
             }
        }
    }

    async fn read_frame(&mut self) -> Result<Option<Command>> {
        let mut decoder = CmppDecoder::default();

        let buf = &mut self.buf;
        loop {
            match self.reader.read_buf(buf).await {
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

