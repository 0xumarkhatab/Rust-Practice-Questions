
pub fn calculate_nth_fib_number(n:u8)->i128{
    let mut a=0;
    let mut b=1;
    let mut c=0;
    if n<1{
        return -1;
    }
    else if n==1{
        return a
    }
    else if n==2{
        return b
    }
    
    for i in 2..n{
        c=a+b;
        a=b;
        b=c;

    }
    
    return c;

}