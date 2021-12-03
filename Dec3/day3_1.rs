use std::io::{self, BufRead};

fn main() {

    let stdin = io::stdin();
    let mut sums: [i32; 12] = [0; 12];
    let mut nl = 0i32;
    for (no_lines, line) in stdin.lock().lines().enumerate() {
        if let Ok(bin_num) = line {
          let chars = bin_num.chars();
          for (i, c) in chars.enumerate(){
            if c == '1' {
              sums[i] += 1;
            }
          }
        }
        nl = no_lines as i32 +1;
      }
      let mut gamma = 0;
      let mut epsilon = 0;
      println!("{:?}", sums);
      for (i, bit) in sums.iter().rev().enumerate(){
        if f64::from(*bit) > (f64::from(nl)/2.0) {
          gamma += 1<<i;
        } else {
          epsilon += 1<<i;
        }
        println!("{} {} {} {} {}", i, bit, f64::from(nl)/2.0, gamma, epsilon);
      }
      println!("gamma {} epsilon {} power {}", gamma, epsilon, gamma*epsilon)
}