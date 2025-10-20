use std::io::{self, Read};
fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    let mut parts = input.trim().split('\n');

    let loop_variable = parts.next().expect("Shouldn't be empty").parse::<u16>().expect("First digit should be an integer");

    println!("Loop variable {}", loop_variable);
}
