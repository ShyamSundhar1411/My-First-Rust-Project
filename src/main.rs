use hello::greet;
fn main() {
    let s1 = String :: from("abc");
    let mut s2 = s1.clone();
    do_stuff(&s1);
    do_another_stuff(&mut s2);
    println!("{}",s2);
}
fn do_stuff(s:&String){
    println!("{}",s);
}
fn do_another_stuff(s: &mut String){
    s.insert_str(0, "Hi,");
}