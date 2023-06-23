#![allow(dead_code)] //允许打印数组
#![allow(unused_variables)] //允许未使用的代码,比如枚举中其中有没有使用的，就不提示
#![allow(non_snake_case)] //允许驼峰写法
use std::thread;
use std::time::Duration;

fn spawn_function() {
    for i in 0..5 {
        println!("spawned thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn main() {
    thread::spawn(spawn_function);

    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
