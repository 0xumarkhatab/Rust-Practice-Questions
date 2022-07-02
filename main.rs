mod fib;
mod greetings;
mod Reader;


pub fn printVectorStats(the_names:Vec<String>){
    println!("\nNames are as Follows := {:?} ",the_names);
    println!("\nTotal Names := {:?} ",the_names.len());
}


fn main() {
    println!("Hello, Rustaceans !");
    print!("\n\t\tFibonacii Numbers Generator \t\t\n");
    println!("7th Fib Bumber is {}",fib::calculate_nth_fib_number(7)); // should print 8
    println!("8th Fib Number is {}",fib::calculate_nth_fib_number(8)); // should print 13 as 8+5 is 13

    print!("\n\t\tConsole Greeting Program \t\t\n");
    greetings::greet();

    print!("\n\t\tHapering with Vectors \t\t\n");
    
    // We are using String here instead of &str because string literals 
    // '&str' are immutable
    
    let mut the_names:Vec<String>=Vec::new();
    
    let mut name=String::from("umar"); //using the String Constructor
    
    // Cloning because of the ownership behaviour of Rust
    the_names.push(name.clone()); 
    printVectorStats(the_names.clone()); 
    
    name=String::from("Seemal");
    the_names.push(name.clone());
    printVectorStats(the_names.clone());    
    
    name=String::from("Ammara");
    the_names.push(name.clone());
    printVectorStats(the_names);
    
    // The Vectors seem Perfectly Working !

  
}
