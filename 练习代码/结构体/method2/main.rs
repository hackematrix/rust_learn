#[derive(Debug)] //debug the code

struct Rectangle{
    width:u32,
    height:u32,
}//结构体

impl Rectangle{
    fn width(&self)->bool{
        self.width>0
    }

    fn height(&self)->bool{
        self.height>0
    }
}


fn main(){
    let rect1=Rectangle{
        width:30,
        height:50,
    };

    if rect1.width(){
        println!("The rectangle has a nonzero width,it is {}",rect1.width());
       
    }

    if rect1.height(){
        println!("The rectangle has a nonzero height,it is {}",rect1.height());
    }

}
