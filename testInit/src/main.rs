fn main() {
    let number = 3;
    if number < 5 {
        println!("条件为 true");
    } else {
        println!("条件为 false");
    }
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
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
