use std::io::{self, Read};
use std::collections::HashMap;
fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    let lines : Vec<_> = input.trim().lines().collect();
    let mut final_classifications : HashMap<i8, i8> = HashMap::new();

    let mut i = 0;
    let mut control = 0;
    let mut highest_scored_contestant = 0;

    for line in lines{
        i += 1;
        let classification_list = line.trim().split_whitespace();
        let mut total : i8 = 0;
        for classification in classification_list {
            if let Ok(c) = classification.parse::<i8>(){
                total += c;
            }
        }

        final_classifications.insert(i, total);

        if total > control {
            control = total;
            highest_scored_contestant = i;
        }
    }
    if!(final_classifications.is_empty()) {
        if let Some(score) = final_classifications.get(&highest_scored_contestant){
            println!("{} {}", highest_scored_contestant, score);
        }
    }
}
