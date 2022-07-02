use crate::Reader::read_console_line;

pub fn greet(){
    println!("\n Enter Your Name := ");
    let mut name = read_console_line() ;
    println!("\nGood Morning, {}",name);

}