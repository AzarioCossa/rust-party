use std::io::{self, Read};
fn add36_minutes (hour : i8, minute : i8) -> (i8, i8){
    if minute + 36 > 59 {
        let m = minute+36-60;
        
        if hour+1 > 23 {
            return (00, m);
        }
        return (hour, m);
    }
    return (hour, minute+36);
}

fn subtract36_minutes (hour : i8, minute : i8) -> (i8, i8){
    if minute - 36 < 0 {
        let m = minute-36+60;
        
        if hour-1 < 0 {
            return (23, m);
        }
        return (hour, m);
    }

    return (hour, minute-36);
}

fn format_time(hour : i8 , minute : i8) -> String{
    let mut h = hour.to_string();
    let mut m = minute.to_string();
    if minute<10 {
        m = format!("0{}", minute);
    }

    if hour < 10  {
        h = format!("0{}", hour);
    }

    return format!("{}:{}", h, m);
}

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    let mut parts : Vec<&str> = input.trim().split("\r\n").collect();

    parts.remove(0);

    println!("");

    for part in parts {
        
       let (mut hour, mut minute) = part.split_once(':').unwrap();
        
       if let Ok(int_hour) = hour.trim().parse::<i8>() && let Ok(int_minute) = minute.trim().parse::<i8>(){
            let (fwd_h , fwd_m) = add36_minutes(int_hour, int_minute);
            let (bwd_h, bwd_m) = subtract36_minutes(int_hour, int_minute);
            print!("{}", format_time(bwd_h, bwd_m));
            println!(" {}", format_time(fwd_h, fwd_m))
       }
    }
}

