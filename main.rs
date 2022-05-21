#![allow(non_snake_case)] // because my folder name is all uppercase
fn main() {
    let mut x = 4; // mutable : because rust is imutaion var
    println!("x is : {}", x);
    x = 5; // or let x = 5; if you don't mut x on the top
    println!("x is : {}", x);

    {
        let x = "alright!!";
        println!("x is : {} because is difference scope", x);
    }

    let x = x + 10; // x is 5 from the top recent line in same scope
    println!("x is : {}", x);

    const SECOND_IN_MIN: u32 = 60; // constant can't change anything
                                   // u32 : The 32-bit unsigned integer type
    println!("second in minute is {}", SECOND_IN_MIN);

    // variable
    let x: u32 = 30; // u32 is a 32-bit only positive number
    let y: f32 = 10.3;
    let b: bool = true;
    if b {
        println!("{} and {}",x,y);
    }

    //tuple, array
    let tub: (i32,bool,&str) = (1, true, "score");
    println!("{}",tub.2); //index

    let mut arr = [1,4,5,6,7];
    arr[0] = 10;
    for c in arr{
        println!("{}",c);
    }
}