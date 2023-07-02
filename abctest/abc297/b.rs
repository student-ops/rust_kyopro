use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut b_num: Option<usize> = None;
    for (i, val) in s.chars().enumerate() {
        if val == 'B' {
            match b_num{
                Some(bn) =>{
                    if ( bn %2) == (i % 2){
                        println!("No");
                        return;
                    }
                }
                None =>{
                    b_num = Some(i);
                    continue;
                }
            }
        }
        if val == 'K'{
            let count = s.chars().skip(i).filter(|&c| c== 'R').count();
            if count != 1{
                println!("No");
                return;
            }
        }

    }
    println!("Yes");
    return;
}
