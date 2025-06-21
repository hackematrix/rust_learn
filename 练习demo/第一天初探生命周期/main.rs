// use std::collections::HashMap;
// fn main() {
//     let query=String::from("name=Venus&age=24&hobby=coding");
//     let qp=QueryParser::from_string(query.clone());
//     println!("params is {:#?}",qp);
// }
// 
// 
// #[derive(Debug)]
// struct QueryParser{
//     query:String,
//     params:HashMap<String,String>
// }
// 
// // methods
// impl QueryParser{
//     fn from_string(query:String)->Self{
//         let params:HashMap<String,String>  =query.split("&")
//             .map(|item|{
//                 let mut parts=item.split("=");
//                 let key=String::from(parts.next().unwrap());
//                 let value=String::from(parts.next().unwrap());
//                 (key,value)
//             }).collect();
// 
//         Self{query,params}
//     }
// }

use std::collections::HashMap;
fn main(){
    let query=String::from("name=Venus&age=24&hobby=coding");
    
    let qb2=EnhanceQueryParser::from_string(&query);
    println!("params is {:#?}",qb2);
    
    let hobby=qb2.params.get("hobby").unwrap();
    println!("hobby is {}",hobby);
}

// 重构
#[derive(Debug)]
struct EnhanceQueryParser<'a>{
    query:&'a str,
    params:HashMap<&'a str,&'a str>
}

// 重构methods
impl<'a> EnhanceQueryParser<'a>{
    pub fn from_string(query:&'a str)->Self{
       let params:HashMap<&'a str,&'a str> =query.split("&")
            .map(|kv|{
                let mut parts=kv.split("=");
                (
                    parts.next().unwrap(),
                    parts.next().unwrap()
                    )
            }).collect();
        
        Self {query,params}
    }
}
