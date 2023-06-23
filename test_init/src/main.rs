#![allow(dead_code)]
#![allow(unused_variables)] //允许未使用的代码,比如枚举中其中有没有使用的，就不提示
#![allow(non_snake_case)]

mod nation {
    pub mod government {
        pub fn govern() {
            println!("I'm govern funtion")
        }
    }
}
//如果模块特别长，层级特别深，可以使用use关键字
use crate::nation::government::govern;
fn main() {
    //这里直接引用use定义的函数
    govern();
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
