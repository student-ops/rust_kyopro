use std::iter::Cloned;

use proconio::input;

fn type_of<T>(_: T) -> String{
    let a = std::any::type_name::<T>();
    return a.to_string();
  }
fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize; n],
    }
    
    let ans :Vec<_>= a.iter()
        .filter(|&x| x < &p)
        .cloned()
        .collect();
    
    println!("{}",type_of(&ans));
    println!("{:?}",ans)
    // println!("{}", ans.len());
}
