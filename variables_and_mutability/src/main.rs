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
}
