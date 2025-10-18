use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();

    let _ = io::stdin().read_to_string(&mut input);

    let mut word_occurences : HashMap<String, u32> = HashMap::new();

    let mut result = String::new();
    let mut highest_occurence_number : u32 = 1;

    for slice in input.trim().split_whitespace(){
        let word=slice.to_string();

        if word_occurences.contains_key(&word){
            let old_occ = *word_occurences.get(&word).unwrap();
            let new_occ = old_occ +1;
            word_occurences.insert(word, new_occ);
            if highest_occurence_number < new_occ as u32 {
                highest_occurence_number = new_occ;
            }
        }else{
            word_occurences.insert(word.clone(), 1 as u32);
        }
    }

    for (word, occurences) in word_occurences{
        if occurences == highest_occurence_number {
            result += &(word+" ");
        }
    }

    println!("\n {}", highest_occurence_number);
    println!("{}", result);
}
