use std::io; // from std(standard lib) import io(input-output)

fn main(){
    // input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line"); // expect : hander the error
    println!("--> {}", input);

    // arithmetic
    let x = 255_u8; // u8 : 0-255
    let y = 127_i8; // -128 - 127 // `#[warn(unused_mut)]` on by default
    println!("{} and {}",x,y);

    let z = x/ (y as u8); // convert type
    println!("{}",z);

    // string to int
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("something error!");
    let int_input: u64 = input.trim().parse().unwrap(); 
    println!("{} plus 10 = {} ",int_input,int_input+10)

}