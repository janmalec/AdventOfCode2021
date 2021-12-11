use std::io::{self, BufRead};
use std::cmp;

fn main() {
    let stdin = io::stdin();
    let mut rakci: Vec<i64> = stdin.lock().lines().next().unwrap().unwrap().split(",").map(|x| x.to_string().parse::<i64>().unwrap()).collect::<Vec<i64>>();
    rakci.sort();
    //let median = rakci[rakci.len()/2];
    //let avg: i64 = rakci.iter().sum::<i64>() / (rakci.len() as i64);
    let mut fuel_spent = i64::MAX;
    
    let mut current_spent;
    for i in 0..rakci.len() {
        current_spent = 0;
        for r in &rakci {
            current_spent += numsum((r-(i as i64)).abs());
        }
        fuel_spent = cmp::min(current_spent, fuel_spent);
    }
    
    println!("{:?}", fuel_spent);

    fn numsum(i: i64) -> i64 {
        let mut sum = 0;
        for j in 0..=i {
            sum += j;
        }
        return sum;
    }
}
