fn main() {
    let number: i32 = 2;
    const FACTOR:f64 = 9.9;
    println!("{},{}",number,FACTOR);
    let x :i32 = 5;
    {
        let x:i32 = 40;
        println!("{}",x);
    }
    println!("{}",x);
}
