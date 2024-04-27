use std::env;
use std::sync::atomic::{AtomicUsize, Ordering};
use log::info;

struct MyStruct {
    field: i32,
}

// 一个独立的函数，用来修改MyStruct实例的field字段
fn modify_my_struct(my_struct: &mut MyStruct, new_value: i32) {
    my_struct.field = new_value;
}

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let foo = AtomicUsize::new(0);
    assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);
    assert_eq!(foo.load(Ordering::SeqCst), 10);

    info!("foo {}", foo.load(Ordering::SeqCst));

    // 创建一个MyStruct的可变实例
    let mut my_instance = MyStruct { field: 42 };
    println!("Before modification: {}", my_instance.field); // 输出：Before modification: 42

    // 调用独立的函数来修改字段的值
    modify_my_struct(&mut my_instance, 100);
    println!("After modification: {}", my_instance.field); // 输出：After modification: 100

    // 继续使用修改后的my_instance实例
    // ...
}