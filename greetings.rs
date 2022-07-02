use crate::reader::read_console_line;

pub fn greet(){
    println!("\n Enter Your Name := ");
    let name = read_console_line() ;
    println!("\nGood Morning, {}",name);

}