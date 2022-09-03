use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let a = 1.1;
    let b = 2.2;
    let c = a + b;
    println!("{} + {} = {}", a, b, c);

    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
