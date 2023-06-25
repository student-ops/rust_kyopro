use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u8,
        p: [u8; n],
        q: [u8; n],
    }

    if p.iter().any(|&x| q.iter().any(|&y| x + y == k)) {
        println!("Yes");
    } else {
        println!("No");
    }
}
