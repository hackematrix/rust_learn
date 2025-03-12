struct Rectangle{
    width:i32,
    height:i32,
}
fn main(){
    let rect1=Rectangle{
       width:30,
       height:40,

    };

    println!("The area of the rectangle is {} square pixels",
    area(&rect1));
}

fn area(rectangle:&Rectangle)->i32{
    rectangle.width*rectangle.height
}
