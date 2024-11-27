fn main(){
    let v:Vec<i32>=(1..=5).collect(); //基于迭代器产生的值来构建向量
    let produit=v.iter().fold(1,|a,b|a*b);//iter()遍历元素
    println!("{}",produit);
}
