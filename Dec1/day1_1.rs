use std::io::{self, BufRead};

fn main() {
    let mut n0 = 0;
    let mut n1;
    let mut count_bigger = -1;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        n1 = line.unwrap().parse::<i32>().unwrap();
        if n1 > n0 {
            count_bigger += 1;
        }
        n0 = n1;
    }
    println!("{}", count_bigger)
}