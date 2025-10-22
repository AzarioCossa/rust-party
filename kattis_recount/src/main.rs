use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    let lines = input.trim().split('\n');

    let mut voting_list = HashMap::new();

    let mut highest_votes = 1;

    let mut winners = Vec::new();

    for line in lines{

        let count = voting_list.entry(line).or_insert(0);

        *count += 1;

        if highest_votes < *count {
                highest_votes = *count;
            }
    }

    for (key, value) in voting_list {
        if value == highest_votes {
            winners.push(key);
        }
    }

    let mut msg = String::new();

    if winners.len() == 1{
        msg = winners[0].to_string();
    }else{
        msg = "Runoff!".to_string();
    }

    println!("{}", msg);
}
