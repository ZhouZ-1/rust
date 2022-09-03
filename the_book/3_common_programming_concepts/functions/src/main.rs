fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // Adding a single semicolon would make the expression a statement.
    // Thankfully rust catches this easily.
    // let y: char = {
    //     let x = 3;
    //     x + 1;
    // };

    // println!("The value of y is: {y}");

    println!("{}", five());

    // loops
    while 1 < 2 {
        println!("again!");
    }
}

fn five() -> i32 {
    let x = 5;
    x
}
