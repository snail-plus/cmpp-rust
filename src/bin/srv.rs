use std::env;
use bytes::BytesMut;
use log::{error, info};
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    // Bind the listener to the address
    // 监听指定地址，等待 TCP 连接进来
    let listener = TcpListener::bind("127.0.0.1:8888").await.unwrap();

    loop {
        // 第二个被忽略的项中包含有新连接的 `IP` 和端口信息
        let (socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async {
            process(socket).await;
        });

    }
}

async fn process(mut socket: TcpStream) {
    // `Connection` 对于 redis 的读写进行了抽象封装，因此我们读到的是一个一个数据帧frame(数据帧 = redis命令 + 数据)，而不是字节流
    // `Connection` 是在 mini-redis 中定义
    let mut buf = BytesMut::with_capacity(1024);
    loop {
        match socket.read_buf(&mut buf).await {
            Ok(n) => {
                info!("read size: {}, str: {}", n, String::from_utf8(buf.to_vec()).unwrap())
            }
            Err(_) => {
                error!("");
                return;
            }
        }
    }
}