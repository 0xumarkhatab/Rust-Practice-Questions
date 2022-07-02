


struct Rectangle{
    width:u32,
    height:u32,

}
impl Rectangle{
    fn area(&self)->u32{
        return self.width*self.height;           
    }
    
}

pub fn main(){
let react_prams=(30,50);  // tuple view of Reactangle Parameters
let rect1 = Rectangle{width:30,
    height:50
    };

println!("\nThe Area of Rectange using Tuple is := {} \n",area(&react_prams));
println!("\nThe Area of Rectange using Struct Method is := {} \n",rect1.area());


}

fn area(rect_prams:&(u32,u32) )->u32{
    return rect_prams.0*rect_prams.1;
    // unable to detect the meaning of indexed data

}

