use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [String; n],
        d: [String; m],
        mut p: [usize; m+1]
    }
    
    // Add a dummy element at the beginning of 'p'

    let mut ans: usize = 0;
    for v in c {
        let mut price = p[0];
        for j in 0..m {
            if v == d[j] {
                price = p[j + 1];
                break;
            }
        }
        ans += price;
    }
    println!("{}", ans);
    return;
}
