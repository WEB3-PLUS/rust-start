fn main() {
    let grade = 1;
    if grade >= 12 && grade <= 20 {
        println!("pass")
    } else if grade >= 7 {
        println!("failed")
    } else {
        println!("ridi");
    }
    //loop
    let mut count = 0;
    'counter: loop {
        if count == 100 {
            println!("nubmer: {count}");
            break 'counter;
        }
        if count % 2 == 0 {
            println!("number: {count}")
        }
        count += 1;
    }

    //and we can use while in rust language

    for element in (1..4).rev() {
        println!("elment: {element}");
    }
}
