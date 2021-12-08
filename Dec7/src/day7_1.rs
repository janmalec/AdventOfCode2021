use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut rakci: Vec<i32> = stdin.lock().lines().next().unwrap().unwrap().split(",").map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    rakci.sort();
    let median = rakci[rakci.len()/2];
    let mut fuel_spent = 0;
    for r in rakci {
        fuel_spent += (r-median).abs()
    }
    println!("{:?}", fuel_spent);
}
