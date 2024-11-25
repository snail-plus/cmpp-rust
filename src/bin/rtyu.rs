use tokio::sync::mpsc::{channel, UnboundedReceiver, UnboundedSender};
use tokio::time::sleep;
use tokio::time::Duration;
use tokio::select;
use std::future::Future;

#[tokio::main]
async fn main() {
    // 创建两个通道
    let (tx1, mut rx1) = channel::<String>(32);
    let (tx2, mut rx2) = channel::<String>(32);

    let tx11 = tx1.clone();
    // 发送一些消息到通道
    tokio::spawn(async move {
        for i in 0..5 {
            tx11.send(format!("Message from channel 1: {}", i)).await.unwrap();
            sleep(Duration::from_secs(1)).await;
        }
    });

    let tx22 = tx1.clone();
    tokio::spawn(async move {
        for i in 0..5 {
            tx22.send(format!("Message from channel 2: {}", i)).await.unwrap();
            sleep(Duration::from_secs(2)).await;
        }
    });

    // 持续监控两个通道的消息
    loop {
        select! {
            result1 = rx1.recv() => {
                if let Some(msg) = result1 {
                    println!("Received: {}", msg);
                    if msg.contains("channel 1") {
                        rx1.close();
                    }
                } else {
                    // 通道已关闭，退出循环
                    break;
                }
            },
            result2 = rx2.recv() => {
                if let Some(msg) = result2 {
                    println!("Received: {}", msg);
                } else {
                    // 通道已关闭，退出循环
                    break;
                }
            },
            // 你可以添加更多的分支来处理其他异步操作
            // _ = other_future() => {
            //     // 处理其他异步操作
            // },
        }
    }
}