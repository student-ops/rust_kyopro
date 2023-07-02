use std::cmp::Ordering;
use proconio::input;

fn main(){
    let mut pairs: Vec<(u128, usize)> = Vec::new();
    input!{
       n:usize 
    }
    for i in 0..n{
        input! {
            a:u128,
            b:u128
        }
        pairs.push(((a*(a+ b)), i));
    }
    
    pairs.sort_by(|&(a1, _), &(a2, _)| a2.cmp(&a1));
    for i in 0..n{
        let (a,b) = pairs[i];
        println!("{} :{}",a,b)
    }

    let indexes: Vec<usize> = pairs.iter().map(|&(_, index)| index).collect();
    
    for index in indexes {
        print!("{} ", index+1);
    }
    println!();
    return;
}
