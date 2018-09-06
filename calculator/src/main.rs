use std::io;

fn main() {
    println!("now a + b problem");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("stdin error");
    let sum: i32 = input.split_whitespace()
                        .map(|x| x.parse::<i32>().expect("parse error"))
                        .sum();
    print!("{}", sum);
}
