use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        H: usize,
        W: usize,
        s: [String; H],
        n: usize,
        e: [(i32, usize, usize); n],
    }

    let map: Vec<Vec<char>> = s.iter().map(|line| line.chars().collect()).collect();

    for i in 0..H {
        println!("{:?}", map[i]);
    }

    println!("{}", map[0][1]);
    println!("{}", e[0].0);
}
