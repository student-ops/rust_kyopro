use proconio::input;

fn main(){
    let mut flag: bool=false;
    input!{
        s:[u32;8]
    }
    flag = s.windows(2).all(|w| w[0] <= w[1]);
    if !flag  {
        println!("No");
        return;
    }
    flag = s.iter().all(|&n| n %25 ==0 && n <= 676 && n >= 100);
    if !flag  {
        println!("No");
        return;
    }
    println!("Yes");
    return;
}
