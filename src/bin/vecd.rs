use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let numbers = vec![];
    // 引用计数指针，指向一个 Vec
    let shared_numbers = Arc::new(Mutex::new(numbers));

    // 循环创建 10 个线程
    let mut hs = Vec::new();
    for i in 0..20 {
        // 复制引用计数指针，所有的 Arc 都指向同一个 Vec
        // move修饰闭包，上面这个 Arc 指针被 move 进入了新线程中
        let arc = shared_numbers.clone();
        let handler = thread::spawn(move || {
            let mut data = arc.lock().unwrap();
            // 我们可以在新线程中使用 Arc，读取共享的那个 Vec
            data.push(i)
        });
        hs.push(handler)
    }

    for h in hs {
        h.join().unwrap();
    }

    println!("{:?}", shared_numbers.lock().unwrap());
}