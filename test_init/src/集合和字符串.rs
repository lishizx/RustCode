#![allow(dead_code)] //允许打印数组
#![allow(unused_variables)] //允许未使用的代码,比如枚举中其中有没有使用的，就不提示
#![allow(non_snake_case)] //允许驼峰写法

fn main() {
    let v = vec![100, 32, 57];
    for i in v {
        println!("{}", i);
    }
    // //集合与字符串
    // let mut list = vec![1, 2, 3, 4, 5, 6, 7, 8];
    // list.push(9);
    // println!("扩展前：{:?}", list);
    // //合并向量
    // let mut list2 = vec![111, 222, 333, 444, 555];

    // list.append(&mut list2);
    // println!("扩展后：{:?}", list);

    // //使用get方法取值
    // println!(
    //     "{}",
    //     match list.get(0) {
    //         //字符串拼接
    //         Some(value) => String::from("取到的值是：") + &value.to_string(),
    //         None => "None".to_string(),
    //     }
    // );
}
