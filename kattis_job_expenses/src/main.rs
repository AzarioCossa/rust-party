use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let values_line = input.split_once('\n').map_or("", |(_n, values)| values);

    let total_expenses = values_line
        .trim()
        .split_whitespace() // Cria um iterador de &str
        .filter_map(|s| s.parse::<i64>().ok()) // Tenta converter para i64, descarta o que falhar
        .filter(|&v| v < 0) // Mantém apenas os números negativos
        .sum::<i64>(); // Soma todos os números restantes

    println!("{}", -total_expenses);
}