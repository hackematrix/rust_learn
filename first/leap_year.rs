use std::io;

fn main(){
    let mut input=String::new();
   io::stdin()
        .read_line(&mut input)
       .expect("Failed to read line");
    let year:i32=input
      .trim()
      .parse()  
       .expect("year entered was not a number");
    if (year%4==0 &&year%100!=0)||year%400==0{
       println!("{} is a leap year",year);
   }
   else{
       println!("{} is not a leap year",year);
  }

}
