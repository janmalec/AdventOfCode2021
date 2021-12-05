use std::io::{self, BufRead};

fn main() {

    let stdin = io::stdin();
    let state = [[0u8; 5]; 5];
    let mut games = Vec::new();
    let mut randoms: Vec<u8> = Vec::new();
    
    let mut new_state = state.clone();
    let mut j = 0;
    for (i, line) in stdin.lock().lines().enumerate() {
        let linestr = line.unwrap().to_string();
        if i == 0 {
            let splits: Vec<_> = linestr.split(",").collect();
            randoms = splits.iter().map(|x| x.parse::<u8>().unwrap()).collect();
            println!("{:?}", randoms);
        } else {
            if linestr.len() == 0 {
                j = 0;
                if i > 2 {
                    games.push(new_state);
                }
            }
            else {
                if j == 0 {new_state = state.clone();}
                let splits: Vec<_> = linestr.split_whitespace().collect();
                let numbers: Vec<u8> = splits.iter().map(|x| x.parse::<u8>().unwrap()).collect();
                for k in 0..numbers.len(){
                    new_state[j][k] = numbers[k];
                }
                j += 1;
            }
        }
    }
    games.push(new_state);
    let mut games_won: Vec<usize> = Vec::new();
    for (i, _ran) in randoms.iter().enumerate(){
        for (j, state) in games.iter().enumerate(){
            if has_won(&state, &randoms, i as usize){
                //println!("WON at step {} sum_unmarked {} lastnum {} multiply {}", i, unmarked_sum(&state, &randoms, i as usize), randoms[i-1], unmarked_sum(&state, &randoms, i as usize)*(randoms[i-1] as i32));
                if !games_won.contains(&j){
                    games_won.push(j);
                }
                if games_won.len() == games.len() {
                    println!("party over at step {} last num {} mul {}", i, randoms[i-1],  unmarked_sum(&state, &randoms, i as usize)*(randoms[i-1] as i32));
                    for k in 0..i {
                        print!{"{} ", randoms[k]};
                    }
                    print!{"\n"};
                    println!("{:?}", &state);
                    break;
                }
            }
        }
        println!("{}/{} {}", games_won.len(), games.len(), i);
        if games_won.len() == games.len() {println!{"over"}; break;}
    }

}

fn has_won(state: &[[u8; 5]; 5], numbers: &Vec<u8>, step: usize) -> bool {
    let mut count_h;
    let mut count_v;
    for i in 0..5{
        count_h = 0;
        count_v = 0;
        for j in 0..5{
            //println!("{} {} {}", i, j, numbers[..step].contains(&32));
            if numbers[..step].contains(&state[i][j]){
                count_h += 1;
                if count_h == 5 {
                    //println!("Won at line {} line is {:?} num {} step {}", i, state[i], state[i][j], step);
                    //println!("{:?}", numbers[step-1]);
                    return true;
                }
            }
            if numbers[..step].contains(&state[j][i]){
                count_v += 1;
                if count_v == 5 {
                    //println!("Won at col {}", j);
                    return true;
                }
            }
        }
    }
    return false;
}

fn unmarked_sum(state: &[[u8; 5]; 5], numbers: &Vec<u8>, step: usize) -> i32 {
    let mut count = 0;
    for i in 0..5{
        for j in 0..5{
            if !numbers[..step].contains(&state[i][j]){
                count += state[i][j] as i32;
            }
        }
    }
    return count;
}