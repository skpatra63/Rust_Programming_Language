#![allow(unused_variables)]

fn main() {
    let width=10;
    let height=4;
    let depth=7;

    let area = area_of(width,height);
    println!("Area is {area}.");

    let vol = volume_of(width,height,depth);
    println!("The volume is {}",vol);


}

fn area_of(x:i32,y:i32) -> i32 {
    x * y
}

fn volume_of(x:i32,y:i32,z:i32) -> i32{
    x*y*z
}