use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut hor = 0;
    let mut dep = 0;
    for line in stdin.lock().lines() {
        if let Ok(l) = line {
            let mut split: Vec<&str> = l.split(" ").collect();
            println!("{:?}", split);
            if split[0] == "forward".to_string(){hor += split[1].parse::<i32>().unwrap();;}
            if split[0] == "down".to_string(){dep += split[1].parse::<i32>().unwrap();;}
            if split[0] == "up".to_string(){dep -= split[1].parse::<i32>().unwrap();;}
            println!("horizontal {} depth {} multiply {}", hor, dep, hor*dep);
        }
    }
}