use std::env;

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let mut line : Vec<char> = args[0].chars().collect();
    line.reverse();
    println!("{}", line.iter().collect::<String>());
}
