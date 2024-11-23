fn binary_search(num:&[i32],target:i32,left:usize,right:usize)->i32
{

    let mid=(left+right)/2;
    if num[mid]==target{
        return mid as i32;
    }
    if num[mid]<target{
        return binary_search(num,target,mid+1,right);
    }
    else {
        binary_search(num,target,left,mid-1);
    }
    -1
}
fn search(num:&[i32],target:i32)->i32
{
    binary_search(num,target,0,num.len()-1)
}
fn main(){
    let nums=vec![-1, 0, 3, 5, 9, 12];
    let target=9;
    let result=search(&nums,target);
    println!("{}",result);
}
