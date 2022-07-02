

pub fn main(){
let react_prams=(30,50);  // tuple view of Reactangle Parameters
println!("\nThe Area of Rectange is := {} \n",area(&react_prams))


}

fn area(react_prams:&(u32,u32) )->u32{
    return react_prams.0*react_prams.1;
    // unable to detect the meaning of indexed data

}


