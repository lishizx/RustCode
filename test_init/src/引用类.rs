#![allow(dead_code)] //允许打印数组
#![allow(unused_variables)] //允许未使用的代码,比如枚举中其中有没有使用的，就不提示
#![allow(non_snake_case)] //允许驼峰写法
mod second;
use second::ClassName;
fn main() {
    let object = ClassName::new(1024);
    object.public_method();
}
