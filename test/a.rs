fn aa(x: i32) -> i32{
    let mut y = x; // this will be copy one and x keep alive
    println!("{}",x);
    y
}

fn main(){
    aa(1);
}