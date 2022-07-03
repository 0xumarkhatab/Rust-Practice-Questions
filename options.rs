

pub fn main(){
    let ch:Option<i32>=None;
    println!("\nTrying Option<i32> with `match` and `if let\n ");
    println!("\t\tMatch Statement Result ");
match ch {
    Some(3) =>{
        println!("Three !")
    }
    _ =>{
        println!("Un-recognized! ");
    }
}

println!("\t\tIf Let Statement Result ");

if let Some(3)=ch {
    print!("Three")
}
else{
    print!("Unrecognized")
}

}