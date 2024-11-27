fn main(){
    let language:Vec<String>=std::env::args().skip(1).collect();
    for l in language{
        println!("{}:{}",l,
            if l.len() %2==0{
                "functional"
            }
            else{
                "imperative"
            });

            
    }
}//运行 cargo run __,__...
