use proconio::input;

fn main() {
    // define i and target at sametime as u8
    let (mut n, mut target):(u8,u8);
    input!{
        n:u8,
        target:u8
    }
    // let mut array: Vec<u8> = Vec::new();

    for _i in 0..n{
        input! {
            i:u8
        }
        if i == target {
            println!("Yes");
            return;
        }
    }
    println!("No");
}