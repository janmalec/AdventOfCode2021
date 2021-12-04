use std::io::{self, BufRead};

fn main() {

    let stdin = io::stdin();
    let mut all = Vec::new();
    for line in stdin.lock().lines(){
        all.push(line.unwrap());
    }
    let mut i = 0;

    let mut first = all.clone();
    while first.len() > 1{
      let criteria = get_criteria(&first, i, 'O');
      let result: Vec<_>  = first.iter().filter(|b| b.chars().nth(i as usize) == Some(criteria)).collect();
      let result: Vec<_>  = result.iter().map(|x| x.to_string()).collect();
      first = result.clone();
      i += 1;
    }
    let mut second = all.clone();
    i=0;
    while second.len() > 1{
      let criteria = get_criteria(&second, i, 'C');
      let result: Vec<_>  = second.iter().filter(|b| b.chars().nth(i as usize) == Some(criteria)).collect();
      let result: Vec<_>  = result.iter().map(|x| x.to_string()).collect();
      second = result.clone();
      i += 1;
    }
    println!("First {} Second {} First int {} Second int {} Multiplied {}", 
  first[0], second[0], binstr_to_int(&first[0]), 
    binstr_to_int(&second[0]), binstr_to_int(&first[0])*binstr_to_int(&second[0]));
}


fn binstr_to_int(s: &String) -> u32 {
  let mut result = 0u32;
  for (i, bit) in s.chars().rev().enumerate(){
    if bit == '1'{
      result +=  1 << i;
    }
  }
  return result;
}

fn get_criteria(v: &Vec<String>, i: i32, quatity: char) -> char {
  let mut count = 0;
  for el in v{
    if el.chars().nth(i as usize) == Some('1') {count += 1};
  }
  if quatity == 'O' {
    if count as f64 >= (v.len() as f64)/2. {return '1'} else {return '0'};
  } else if quatity == 'C' {
    if (count as f64) < (v.len() as f64)/2. {return '1'} else {return '0'};
  } else {
    panic!("Illegal option");
  }
}