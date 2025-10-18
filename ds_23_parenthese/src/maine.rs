use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read input");

    let mut results = Vec::new();

    for expr in input.lines() {
        let mut no_open_pars = 0;
        let mut balanced = true;

        for c in expr.chars() {
            if c == '(' {
                no_open_pars += 1;
            } else if c == ')' {
                no_open_pars -= 1;
                if no_open_pars < 0 {
                    balanced = false;
                    break;
                }
            }
        }

        if no_open_pars != 0 {
            balanced = false;
        }

        results.push(if balanced { "Oui" } else { "Non" });
    }

    for result in results {
        println!("{}", result);
    }
}
