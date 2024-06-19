use std::sync::{Arc, Mutex};
use std::thread::yield_now;
use std::time::Duration;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mutex = Arc::new(Mutex::new(0));
    let arc1 = Arc::clone(&mutex);
    let arc12 = Arc::clone(&mutex);

    let handle1 = tokio::spawn(async {
        a(arc1).await
    });

    let handle2 = tokio::spawn(async {
        b(arc12).await
    });

    handle1.await?;
    handle2.await?;
    Ok(())
}

async fn a(c: Arc<Mutex<i32>>) {
    loop {
        tokio::time::sleep(Duration::from_secs(3)).await;
        let mut data = c.lock().unwrap();
        *data += 1;
        println!("a, data: {}", *data)
    }
}

async fn b(c: Arc<Mutex<i32>>) {
    loop {
        tokio::time::sleep(Duration::from_secs(3)).await;
        let mut data = c.lock().unwrap();
        *data += 1;
        println!("b, data: {}", *data)
    }
}