use std::env;
use std::io::{self, Read};

fn main() {

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read input");
    
    let mut noOpenPars = 0;
    let mut results: Vec<String> = Vec::new();

    for expr in input.lines() {

        let expr_chars : Vec<char> = expr.chars().collect();

        for c in expr_chars {
            if c == '(' {
                noOpenPars += 1;
            } else if c == ')' {
                noOpenPars -= 1;
            }
        }

        if noOpenPars == 0 {
            results.push("Oui".to_string());
        } else {
            results.push("Non".to_string());
        }

    }

    for result in results {
        println!("{}", result);
    }
}