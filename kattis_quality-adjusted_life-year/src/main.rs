use std::io::{self, Read};
fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    let mut lines : Vec<&str>= input.trim().lines().collect();

    lines.remove(0);

    let mut life_quality_sum : f32 = 0.0;

    for line in lines{
        let (life_quality, time_span) = line.split_once(" ").unwrap();

        if let Ok(quality) = life_quality.trim().parse::<f32>() {
            if let Ok(time) = time_span.trim().parse::<f32>(){
                life_quality_sum += quality * time;
            }
        }
    }
    println!("{}", life_quality_sum);
}
