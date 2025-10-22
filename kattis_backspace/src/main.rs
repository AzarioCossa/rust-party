use std::io::{self, Read};
fn remove_extras (expression : &str) -> String{
    let mut expression_chars = Vec::new();

    for c in expression.chars(){
        if c =='<' {
            expression_chars.pop();
        }else{
            expression_chars.push(c);
        }
    }

    return expression_chars.into_iter().collect();
}
fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);


    let expression = input.trim();

    let new_expression = remove_extras(expression);

    println!("{}", new_expression);
}
