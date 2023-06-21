#![allow(dead_code)]
#![allow(unused_variables)] //允许未使用的代码,比如枚举中其中有没有使用的，就不提示
enum Book {
    Papery { index: u32 },
    Electronic { url: String },
}
enum Option<T> {
    Some(T),
    None,
}
fn main() {
    let opt = Option::Some("hello");

    match opt {
        Option::Some(something) => {
            println!("{}", something);
        }
        Option::None => {
            println!("op is nothing");
        }
    }
    // let t = "abc";
    // match t {
    //     "abc" => println!("okk"),
    //     _ => {}
    // }

    // let book: Book = Book::Papery { index: 1001 };

    // match book {
    //     Book::Papery { index } => {
    //         println!("Papery book {}", index);
    //     }
    //     Book::Electronic { url } => {
    //         println!("E-book {}", url);
    //     }
    // }

    // let book = Book::Papery { index: 1001 };
    // let ebook = Book::Electronic {
    //     url: String::from("url..."),
    // };

    // match book {
    //     Book::Papery { index } => {
    //         println!("Papery book {}", index);
    //     }
    //     Book::Electronic { url } => {
    //         println!("E-book {}", url);
    //     }
    // }

    // match ebook {
    //     Book::Papery { index } => {
    //         println!("Papery book {}", index);
    //     }
    //     Book::Electronic { url } => {
    //         println!("E-book {}", url);
    //     }
    // }

    // struct Rectangle {
    //     width: u32,
    //     height: u32,
    // }
    // impl Rectangle {
    //     fn area(&self) -> u32 {
    //         self.width * self.height
    //     }
    // }

    // let resct = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    // println!("this mainji is {}", resct.area());
    // struct Cat {
    //     name: String,
    //     sex: String,
    //     from: String,
    // }
    // let a_new_cat: Cat = Cat {
    //     name: String::from("老白的猫"),
    //     sex: String::from("母"),
    //     from: String::from("菜市场买的"),
    // };

    // println!("{:?}", a_new_cat);
    // struct Color(u8, u8, u8);

    // let black = Color(6, 7, 4);

    // println!("{}", black.0);
    // println!("{}", black.1);
    // println!("{}", black.2);

    // println!("{}", a_new_cat.sex);
    // println!("{}", a_new_cat.from);
    // let s = String::from("HeiHei");
    // println!("{}", s);
    //取值
    // let StringArr = ["大白", "小爱", "大鹏", "小琴"];
    // for item in StringArr.iter() {
    //     println!("item: {}", item);
    // }

    // let mut number = 1;
    // while number != 4 {
    //     println!("{}", number);
    //     number += 1;
    // }
    // println!("EXIT");
    // let number = 3;
    // if number < 5 {
    //     println!("条件为 true");
    // } else {
    //     println!("条件为 false");
    // }
    // fn five() -> i32 {
    //     5
    // }
    // println!("five() 的值为: {}", five());
    // let x = 5;

    // let y = {
    //     let x = 3;
    //     x + 1
    // };

    // println!("x 的值为 : {}", x);
    // println!("y 的值为 : {}", y);
    // let mut d: [i32; 5] = [3; 5];
    // d[0] = 777;
    // println!("{:?}", d);
    // let number = [1, 2, 3, 4, 5];

    // println!("{}", number[0]);
    // println!("{}", number[1]);
    // println!("{}", number[2]);
    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    // let (x, y, z) = tup;
    // println!("{}", x);
    // println!("{}", y);
    // println!("{}", z);

    // let flag = false;
    // println!("{}", flag);
}

//两个数值相加
// fn add(a: i32, b: i32) -> i32 {
//     return a + b;
// }
