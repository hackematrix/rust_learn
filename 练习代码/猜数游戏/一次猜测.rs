use std::io; //prelude
fn main() {
    println!("猜数");

    println!("猜一个数");

    let mut guess=String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("你猜测的数是:{}",guess);

}
