#[derive(Debug)]
struct Solution;

impl Solution{
    pub fn sum(num1:i32,num2:i32)->i32{
        num1+num2
    }
}



fn main(){
    let ans=Solution::sum(12,5);
    println!("{}",ans);
}
