

pub fn simulate(){
    println!("\t\t-- Welcome to Rust Interaction Database -- \n");
    println!("Select the Operation to Perform ! \n");
    println!("1-> Add Record \n2-> Remove Record \n3-> Print Records\n4->Exit");
    let choice=0;
    match choice {
        1=>{
            println!("\nEnter Your name:= ");
            let record =read_console_line();
            the_names = insert_record(record, the_names.clone());
            println!("\n\t\tRecord Has Been Added Successfully !....\n");

        },
        2=> {
            println!("Removing ! ..")
        },
        3=>{
            println!("Printing ! ..")
        },
        4=>{
            println!("Exitting !")
        }
        _=>println!("invalid")

        
    }


}
fn insert_record(record:String,the_names:Vec<String> )->Vec<String>{

    let mut name=String::from(record); //using the String Constructor
    // Cloning because of the ownership behaviour of Rust
    let mut state = the_names.clone();
    state.push(name.clone()); 
    
    return state;
    
}
