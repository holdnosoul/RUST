
fn main() {
    println!("Hello, world!");
    println!("{:?}",ab_i32(11,21));
    println!("{:?}",ab_f64(1.1,2.1));
    println!("{:?}",ab_T(1.1,2.1));

}


fn ab_i32(a:i32,b:i32)->i32{
    if a>b {return a;}
    else {return b;}
}

fn ab_f64(a:f64,b:f64)->f64{
    if a>b {return a;}
    else {return b;}
}

fn ab_T<T: PartialOrd + Copy>(a:T,b:T) -> T {
    if a>b {return a;}
    else {return b;}
}
