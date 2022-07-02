#![allow(dead_code)]
#![allow(unused_variables)]

mod fib;
mod greetings;
mod reader;
mod database_system;
mod react_area_structs_practice;


fn main() {
    println!("Hello, Rustaceans !");
    print!("\n\t\tFibonacii Numbers Generator \t\t\n");
    println!("7th Fib Bumber is {}",fib::calculate_nth_fib_number(7)); // should print 8
    println!("8th Fib Number is {}",fib::calculate_nth_fib_number(8)); // should print 13 as 8+5 is 13

 //  print!("\n\t\tConsole Greeting Program \t\t\n");
 //   greetings::greet();
    // println!("\nTiny database");
    // database_system::simulate();
    println!("\nReactangle Area : \n");
    react_area_structs_practice::main();

  
}
