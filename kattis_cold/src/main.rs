use std::io::{self, Read};
use std::collections::HashSet;
fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let mut tempertatures_list = Vec::new();
    tempertatures_list = input.trim().split_whitespace().collect();

    let mut negative_temperatures_total : u16 = 0;

    for temperature in tempertatures_list {
        if temperature.contains("-") {
            negative_temperatures_total += 1;
        }    
    }

    println!("{}", negative_temperatures_total);
}
