use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::{error, info};
use cmpp::server::Config;
use cmpp::server::server::Server;
use tokio::{io, signal};


#[tokio::main]
async fn main() -> io::Result<()> {
    // 创建一个日志构建器
    let mut builder = Builder::new();

    // 设置日志格式，使用 chrono 获取本地时间
    builder.format(|buf, record| {
        let local_time = Local::now().format("%Y-%m-%d %H:%M:%S");
        writeln!(
            buf,
            "{} [{}] {} {}",
            local_time,
            record.level(),
            record.target(),
            record.args()
        )
    });

    // 设置日志级别
    builder.filter(None, log::LevelFilter::Info);

    // 初始化日志记录器
    builder.init();

    let cfg = Config::default();
    let mut srv = Server::new(cfg).await?;

    tokio::select! {
        res = srv.run() => {
            // If an error is received here, accepting connections from the TCP
            // listener failed multiple times and the server is giving up and
            // shutting down.
            //
            // Errors encountered when handling individual connections do not
            // bubble up to this point.
            if let Err(err) = res {
                error!("failed to accept: {:?}", err);
            }
        }
        _ = signal::ctrl_c() => {
            // The shutdown signal has been received.
            info!("shutting down");
        }
    }

    Ok(())
}
