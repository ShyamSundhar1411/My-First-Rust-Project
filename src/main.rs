use hello::greet;
fn main() {
    greet();
    let num:i32 = 5;
    let mut msg:&str;
    msg = if num==5{
        "five"
    }
    else {
        "four"
    };
    println!("{}",msg);
}
