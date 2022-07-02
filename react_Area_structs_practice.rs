


struct Rectangle{
    width:u32,
    height:u32,

}
pub fn main(){
let react_prams=(30,50);  // tuple view of Reactangle Parameters
let params = Rectangle{width:30,
    height:50
    };

println!("\nThe Area of Rectange using Tuple is := {} \n",area(&react_prams))
println!("\nThe Area of Rectange using Struct is := {} \n",area_through_struct(&params))


}

fn area(rect_prams:&(u32,u32) )->u32{
    return rect_prams.0*rect_prams.1;
    // unable to detect the meaning of indexed data

}

fn area_through_struct(rect_prams:&Rectangle)->u32{
    return rect_prams.width*rect_prams.height;
    
    // More Meaningful Calculation

}


