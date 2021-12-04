use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut hor = 0;
    let mut dep = 0;
    let mut aim = 0;
    for line in stdin.lock().lines() {
        if let Ok(l) = line {
            let mut split: Vec<&str> = l.split(" ").collect();
            println!("{:?}", split);
            if split[0] == "forward".to_string(){
                let x = split[1].parse::<i32>().unwrap();
                hor += x;
                dep += aim*x;
            }
            if split[0] == "down".to_string(){aim += split[1].parse::<i32>().unwrap();;}
            if split[0] == "up".to_string(){aim -= split[1].parse::<i32>().unwrap();;}
            println!("horizontal {} depth {} aim {} multiply {}", hor, dep, aim, hor*dep);
        }
    }
}