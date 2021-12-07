use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut ribice: Vec<i32> = stdin.lock().lines().next().unwrap().unwrap().split(",").map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    //println!("{:?}", &ribice);
    
    let days = 80;
    let mut nove_ribice;
    for i in 0..days {
        nove_ribice = 0;
        for ribica in ribice.iter_mut() {
            if *ribica > 0 {*ribica -= 1;}
            else if *ribica == 0 {*ribica = 6; nove_ribice += 1;}
        }
        ribice.append(&mut vec![8i32; nove_ribice]);
    }
    println!("{}", ribice.len());
}