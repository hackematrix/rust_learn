use std::io;
use rand::Rng; //trait

fn main(){
    println!("猜数游戏 !!");

    let secret_number=rand::thread_rng().gen_range(1, 101);
    println!("神秘数字是{}",secret_number);

    println!("猜一个数");

    let mut guess=String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("你猜的数字是:{}",guess);



}
