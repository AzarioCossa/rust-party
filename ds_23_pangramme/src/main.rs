use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    
    let alphabet : HashSet<_>="abcdefghijklmnopqrstuvwxyz".chars().collect();

    for line in input.lines(){
        let mut missing_characters = Vec::new();
        let mut line_chars : HashSet<_>= line.chars().collect();
        line_chars = line_chars.into_iter().flat_map(|c| c.to_lowercase()).collect();
        
        if line_chars.is_superset(&alphabet){
            println!("{}","Pangramme".to_uppercase());
        }else{
            for c in alphabet.clone(){
                if !line_chars.contains(&c){
                    missing_characters.push(c.clone().to_string()); 
                }
            }
            missing_characters.sort();
            println!("{:?}",missing_characters.into_iter().collect::<String>());
        }
    }
}
