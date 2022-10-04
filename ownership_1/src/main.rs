
fn main() {
    // println!("Hello, world!");
     let mut s1 = String::from("abc");
    // let _s2=s1.clone();
    s1 = do_stuff(s1);
    println!("{}",s1);
}
fn do_stuff(s: &String)-> String{
    // do stuff
    s
}