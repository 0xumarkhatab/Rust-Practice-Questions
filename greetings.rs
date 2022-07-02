use crate::Reader::read_console;

pub fn greet(){
    println!("\n Enter Your Name := ");
    let mut name = read_console() ;
    println!("\nGood Morning, {}",name);

}