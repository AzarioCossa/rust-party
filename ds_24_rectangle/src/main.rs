use std::io::{self, Read};


fn main() {
    let mut input = String::new();

    let _ = io::stdin().read_to_string(&mut input);

    let mut parts = input.trim().split_whitespace();

    let height = parts.next().expect("Height shouldn't be empty").parse::<u16>().expect("Height should be a number");
    let width = parts.next().expect("Width shouldn't be empty").parse::<u16>().expect("Width should be a number");

    for _ in 0..height{
        let mut word =String::new();
        for __ in 0..width{
            word +="x";
        }
        println!("{}", word);
    }
}
