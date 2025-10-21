use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    let lines = input.trim().split('\n');

    let mut tagged_lines = String::new();
    let mut line_counter = 0;

    for line in lines {
        line_counter += 1;
        if line.contains("FBI"){
            tagged_lines = tagged_lines + &(format!(" {}", line_counter));
        }
    }

    if tagged_lines==""{
        tagged_lines = "HE GOT AWAY!".to_string();
    }

    println!("{}",  &tagged_lines);
}
