use std::io::{self, BufRead};

extern crate matrix;
use std::cmp;

use matrix::prelude::*;

fn main() {
    #[derive(Debug)]
    struct Point {x: usize, y: usize}
    #[derive(Debug)]
    struct Line {p1: Point, p2: Point}

    let mut lines: Vec<Line> = Vec::new();
    let mut max = Point{x: 0, y: 0};

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let ln = line.unwrap();
        let coords = ln.replace("->", ",");
        let num_str: Vec<String> = coords.split(",").map(|x| x.trim().to_string()).collect();
        let n: Vec<usize> = num_str.iter().map(|x| x.parse::<usize>().unwrap()).collect();
        lines.push(Line{p1: Point{x: n[0], y: n[1]}, p2: Point{x: n[2], y: n[3]}});
        max.x = cmp::max(max.x, n[0]);
        max.x = cmp::max(max.x, n[2]);
        max.y = cmp::max(max.y, n[1]);
        max.y = cmp::max(max.y, n[3]);
        println!("{:?}", lines.last());
    }
    println!("{:?}", max);
    let mut field = Compressed::<usize>::zero((max.x, max.y));
    for line in lines {
        if line.p1.x == line.p2.x {
            let y1 = std::cmp::min(line.p1.y, line.p2.y);
            let y2 = std::cmp::max(line.p1.y, line.p2.y);
            for y in y1..=y2 {
                field.set((line.p1.x, y), &field.get((line.p1.x, y))+1);
            }
        }
        else if line.p1.y == line.p2.y {
            let x1 = std::cmp::min(line.p1.x, line.p2.x);
            let x2 = std::cmp::max(line.p1.x, line.p2.x);
            for x in x1..=x2 {
                field.set((x, line.p1.y), &field.get((x, line.p1.y))+1);
            }
        }        
    }
    let mut overlaps = 0;
    for v in field.values {
        //rint!("{:?} ", v);
        if v > 1 {overlaps += 1;}
    }

    for x in 3..=1 {
        println!("{}", x);
    }
    println!("Overlaps: {}", overlaps);
}