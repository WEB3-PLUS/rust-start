use std::io;

fn main() {
    // let x = 5; imutable variable
    let mut x: i32 = 5; //mutable variable
    println!("x value is : {x}");
    x = 6;
    println!("x value is : {x}");

    //constant
    const ONE_DAY_IN_SEC: u64 = 24 * 60 * 60;
    println!("one day in secconde: {ONE_DAY_IN_SEC}");

    //shadowing
    let y = 5;

    let y = y + 1;
    {
        let y = y * 2;
        println!("y value is: {y}");
    }
    println!("y value is : {y}");

    //dataTypes
    //rust is a statically
    let age = "18";
    let age: u8 = age.parse().expect("type is not casting to string"); //type casting string to uint8bit

    println!("age is : {age}");

    //scalat types

    //integer
    //boolean
    //string

    //compound Types

    //tuple
    let _tup: (&str, u8, bool) = (&"mohammadreza", 2, false);
    //array
    let names = ["mohammadreza", "reza"];
    let ages: [u8; 2] = [29, 10];
    let role = ["test"; 10];

    //array test

    let a = [1, 2, 3, 4, 5];

    println!("please enter array index : ");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");
    let index: usize = index.trim().parse().expect("index entered is not a number");

    let element = a[index];

    println!("the index value is {element}");
}
