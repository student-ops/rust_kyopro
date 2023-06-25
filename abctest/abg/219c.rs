use::proconio::input;

fn main(){
    input! {
        x: [usize;26],
    }
    let y :Vec<_> = x.iter()
        .map(|&a| {
            30 -a
        })
        .collect();

    println!("{:?}",y);
    
}