use std::io::{self, Read};


fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let mut sum : Vec<f32> = Vec::new();

    for line in input.trim().split("\n"){
        let mut line_sum = 0.0;
        for slice in line.split_whitespace(){
            if let Ok(number)=slice.parse::<f32>(){
                if number.fract()==0.0{
                    line_sum += number;
                }
            }
        }
        sum.push(line_sum);
    }
    
    println!("\n");

    for element in sum {
        println!("{}", element);
    }

}
