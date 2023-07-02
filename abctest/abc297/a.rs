use proconio::input;

fn main(){
    input!{
        n:usize,
        d:u32,
        t:[u32;n]
    }
    for a in 1..n{
       if t[a] -  t[a-1] <= d {
        println!("{}",t[a]);
        return;
       }
    }
    println!("-1");
    return;
    
}