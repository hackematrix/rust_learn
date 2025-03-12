#[derive(Debug)]

struct Rectangle{
    width:u32,
    height:u32,
}

fn main(){
    let scale=2;
    let rec=Rectangle{
        width:dbg!(30*scale),
        height:50,
    };
    dbg!(&rec);
}
