use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    //                     0  1  2  3  4  5  6  7  8  9
    //let xs: [i32; 10] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];
    //let result: [i32; 10] = [0; 10];
    let mut count = 0;
    for line in stdin.lock().lines() {
        let ln = line.unwrap().to_string();
        let split: Vec<String> = ln.split('|').map(|s| s.to_string()).collect();
        let _numbers: Vec<String> = split[0].trim().split_ascii_whitespace().map(|s| s.to_string()).collect();
        let display: Vec<String> = split[1].trim().split_ascii_whitespace().map(|s| s.to_string()).collect();
        for digit in display {
            if (digit.len() == 2) | (digit.len() == 4) | (digit.len() == 3) | (digit.len() == 7) {
                count += 1;
            } 
        }
        //break;
    }
    println!("{}", count);
    //let mut vrstica: Vec<i32> = stdin.lock().lines().next().unwrap().unwrap().split("|").collect::<Vec<String>>();

}
