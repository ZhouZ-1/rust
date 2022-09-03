use std::io;

fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    // Accessing tuple elements using a period
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!(
        "The value of x is: ({}, {}, {})",
        five_hundred, six_point_four, one
    );

    // Accessing tuple elements using bracket notation
    // Throws error
    // let five_hundred = x[0];
    // let six_point_four = x[1];
    // let one = x[2];

    // println!(
    //     "The value of x is: ({}, {}, {})",
    //     five_hundred, six_point_four, one
    // );

    // Arrays

    let _a = [1, 2, 3, 4, 5];

    let a = [3; 5];

    let first = a[0];
    let second = a[1];

    println!("The value of a is: ({}, {})", first, second);

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
