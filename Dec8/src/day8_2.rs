use std::io::{self, BufRead};
use std::collections::{HashSet,HashMap};
use std::iter::FromIterator;

/*
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
*/

fn main() {
    let stdin = io::stdin();
    //                     0  1  2  3  4  5  6  7  8  9
    //let xs: [i32; 10] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];
    //let result: [i32; 10] = [0; 10];
    let mut count = 0;
    for line in stdin.lock().lines() {
        let ln = line.unwrap().to_string();
        let split: Vec<String> = ln.split('|').map(|s| s.to_string()).collect();
        let mut numsRead: Vec<String> = split[0].trim().split_ascii_whitespace().map(|s| s.to_string()).collect();
        let display: Vec<String> = split[1].trim().split_ascii_whitespace().map(|s| s.to_string()).collect();
        let mut options: HashMap<char, HashSet<char>> = HashMap::new();
        let mut map: HashMap<char, char> = HashMap::new();
        
        numsRead.sort_by(|b, a| b.len().cmp(&a.len()));
        //println!("{:?}", numsRead);
        let mut numbers: Vec<HashSet<char>> = numsRead.iter().map(|x| HashSet::from_iter(x.chars())).collect();
        
        let a: char = *numbers[1].difference(&numbers[0]).next().unwrap();
        println!("{:?}", a);

        for num in &numbers{
            if num.len() == 6 {
                let mut dif =  numbers[1].difference(&num);
                if &dif.count() == &1{
                    //let c: char = dif.next().unwrap();
                }
                //println!("{:?}", dif);
            }
        }


        break;
    }
}
