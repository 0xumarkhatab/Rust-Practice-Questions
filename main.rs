mod fib;
mod Reader;



fn main() {
    println!("Hello, Rustaceans !");
    print!("\n\t\tFibonacii Numbers Generator \t\t\n");
    println!("7th Fib Bumber is {}",fib::calculate_nth_fib_number(7)); // should print 8
    println!("8th Fib Number is {}",fib::calculate_nth_fib_number(8)); // should print 13 as 8+5 is 13

    

}
