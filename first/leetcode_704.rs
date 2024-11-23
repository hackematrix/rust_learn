struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32
    {
        let mut low=0;
        let mut high=nums.len() as isize -1;
        while low<=high{
            let  mid=(low+high)/2;
            if nums[mid as usize]==target{
                return mid as i32;
            }
            else if nums[mid as usize]<target{
                low=mid+1;
            }
            else{
                high=mid+1;
            }
        }
        -1
    }
}//二分查找
fn main(){
    let nums=vec![-1,0,3,5,9,12];
    let target=9;
    let result=Solution::search(nums,target);
    println!("{}",result);

}
