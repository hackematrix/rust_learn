use std::io;
use std::io::Write;

fn main(){
    let mut input=TyperComputer::new(CommandLineComputer);
    loop{
        input.type_expression();
        println!("Result is {}",input.compute());
    }
}

// 接口
pub trait Computer{
    fn compute(&self,expression:&str)->i32;
}

struct CommandLineComputer;

impl Computer for CommandLineComputer{
    fn compute(&self,expression:&str)->i32{
        let mut num1=String::new();
        let mut num2=String::new();
        let mut operator:Option<char> =None; //运算符号
        
        for char_index in expression.trim().chars(){
            // 赋值给num1和num2
            if char_index.is_digit(10){
                if operator.is_none(){
                    num1.push(char_index);
                } else{
                    num2.push(char_index);
                }
                continue;
            }
            
        // 赋值给operator
            match char_index{
                '+' | '-' | '/' | '*' if operator.is_none() =>operator=Some(char_index),
                _ if char_index.is_whitespace() =>continue,
                _ =>panic!("Invalid character:{}",char_index)
            }
        }
        
        if num1.is_empty() | num2.is_empty() | operator.is_none() {
            panic!("Invalid character:{}",expression);
        }
        
    // num1,num2,operator解包
        let num1=num1.parse::<i32>().unwrap();
        let num2=num2.parse::<i32>().unwrap();
        let operator=operator.unwrap();
        
        match operator{
            '+' =>num1+num2,
            '-' =>num1-num2,
            '*' =>num1*num2,
            '/' =>num1/num2,
            _=>unreachable!()
        }
    }
}

struct TyperComputer<T:Computer>{
    command_compute:T,
    expression:String,
}

impl<T:Computer> TyperComputer<T>{
    fn new(command_compute:T)->Self{
        Self{command_compute,expression:String::new()}
    }
    
    fn type_expression(&mut self){
        self.expression.clear();
        println!("Please enter an expression");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut self.expression);
    }
    
    fn compute(&mut self)->i32{
        self.command_compute.compute(&self.expression)
    }
}
    
