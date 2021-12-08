use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let ribice: Vec<i64> = stdin.lock().lines().next().unwrap().unwrap().split(",").map(|x| x.to_string().parse::<i64>().unwrap()).collect::<Vec<i64>>();
    
    let days = 256;
    let mut nove_ribice;
    let mut histogram = [0i64; 9];
    for ribica in ribice {
        histogram[ribica as usize] += 1;
    }
    for _i in 0..days {
        nove_ribice = histogram[0];
        for j in 0..(histogram.len()-1) {
            histogram[j] = histogram[j+1];
        }
        histogram[6] += nove_ribice; 
        histogram[8] = nove_ribice; 
    }
    println!("{}", histogram.iter().sum::<i64>());
}