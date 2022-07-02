use crate::Reader::{read_console_line, read_int};


pub fn simulate(){
    print!("\n\t\tHapering with Vectors \t\t\n");
    
    // We are using String here instead of &str because string literals 
    // '&str' are immutable
    
    let mut the_names:Vec<String>=Vec::new();
    let mut choice:i32 =10;
    const EXIT_CHOICE:i32 = 4;
    while choice!=EXIT_CHOICE{
        println!("\t\t-- Welcome to Rust Interaction Database -- \n");
        println!("Select the Operation to Perform ! \n");
        println!("1-> Add Record \n2-> Remove Record \n3-> Print Records\n4->Exit");
        choice= read_int();
        match choice {
            1=>{
                println!("\nEnter Record name:= ");
                let record =read_console_line();
                the_names = insert_record(record, the_names.clone());
                println!("\n\t\tRecord Has Been Added Successfully !....\n");
            },
            2=> {
                println!("\nRemoving Record  ");
    
            },
            3=>{
                printDatabase(the_names.clone());
            },
            EXIT_CHOICE=>{
                println!("\n\t\tExitting !... ");
            }
            _=>println!("invalid")
    
            
        }
    
    }
    



}

fn insert_record(record:String,the_names:Vec<String> )->Vec<String>{

        let mut name=String::from(record); //using the String Constructor
        // Cloning because of the ownership behaviour of Rust
        let mut state = the_names.clone();
        state.push(name.clone()); 
        
        return state;
        
}

fn printDatabase(db:Vec<String>){
    println!("\n\n\t\t---- Printing Database State ---- \n");
    let mut index=1;
    for element in db.iter(){
        println!("{} -> {}",index,element);
        index=index+1;
    }

}
