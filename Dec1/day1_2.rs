use std::io::{self, BufRead};
use std::collections::VecDeque;

fn main() {
    let mut sum1 = VecDeque::<i32>::with_capacity(3);
    let mut sum2 = VecDeque::<i32>::with_capacity(3);
    let mut n1;
    let mut count_bigger = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        n1 = line.unwrap().parse::<i32>().unwrap();
        sum1.push_front(n1);
        if sum1.len() == 3 && sum2.len() == 3 {
            let s1: i32 = sum1.iter().sum();
            let s2: i32 = sum2.iter().sum();
            println!("{:?} {:?} {} {}", sum1, sum2, s1, s2);
            if s1 > s2 {count_bigger += 1;}
        };
        if sum1.len() == 3 {sum1.pop_back();}
        if sum2.len() == 3 {sum2.pop_back();}
        sum2.push_front(n1);
    }
    println!("{}", count_bigger);
}