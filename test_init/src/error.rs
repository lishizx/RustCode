#![allow(dead_code)]
#![allow(unused_variables)] //允许未使用的代码,比如枚举中其中有没有使用的，就不提示
#![allow(non_snake_case)]

use std::io;
// mod nation {
//     pub mod government {
//         pub fn govern() {
//             println!("I'm govern funtion")
//         }
//     }
// }
// //如果模块特别长，层级特别深，可以使用use关键字
// use crate::nation::government::govern;
fn f(i: i32) -> Result<i32, bool> {
    if i >= 0 {
        Ok(i)
    } else {
        Err(false)
    }
}

fn main() {
    let r: Result<i32, bool> = f(10000);
    if let Ok(v) = r {
        println!("Ok: f(-1) = {}", v);
    } else {
        println!("Err");
    }
    // let f1 = File::open("hello1.txt").unwrap();
    // let f2 = File::open("hello1.txt").expect("读取失败");
    // let f = File::open("hello.txt");
    // match f {
    //     Ok(file) => {
    //         println!("打开文件成功！{:?}", file);
    //     }
    //     Err(err) => {
    //         println!("打开文件失败！");
    //     }
    // }

    //这里直接引用use定义的函数
    // govern();
    //如果模块特别长，层级特别深，可以使用use关键字
    // let person = SomeModule::Person::King {
    //     name: String::from("我是蓝色"),
    // };

    // match person {
    //     SomeModule::Person::King { name } => {
    //         println!("{}", name)
    //     }
    //     _ => {} //如果只是指定某个，其他的使用这个当省略处理
    // }
}
