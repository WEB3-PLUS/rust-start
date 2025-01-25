use std::io;

fn sum(a: i32, b: i32) -> bool {
    let sum = a + b;
    println!("sum: {sum}");
    return true;
}
fn main() {
    println!("Hello, world!");
    let mut a = String::new();
    let mut b = String::new();
    io::stdin().read_line(&mut a).expect("value is not number");
    io::stdin().read_line(&mut b).expect("value is not number");
    let a: i32 = a.trim().parse().expect("error");
    let b: i32 = b.trim().parse().expect("error");
    sum(a, b);
}
