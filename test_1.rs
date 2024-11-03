//use std::io;
//
//fn main(){
//    let mut input=String::new();
//    io::stdin()
//        .read_line(&mut input)
//        .expect("Failed to read line");
//    let c:i32=input
//        .trim()
//        .parse()
//        .expect("input enter was not a number");
//    let  f:f64;
//    f=c as f64*(9.0/5.0)+32.0;
//    println!("{}",f);
//
//}


use std::io;

fn main(){
    let mut input=String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let num:i32=input
        .trim()
        .parse()
        .expect("input extered was not a number");
    let a:i32;
    let b:i32;
    let c:i32;
    a=num%10;
    b=num/10%10;
    c=num/100;
    println!("{} {} {}",a,b,c);


}
