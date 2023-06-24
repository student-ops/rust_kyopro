use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let mut masu: Vec<(i32, i32, i32)> = vec![(0, 0, 0)];
    let mut yes: bool = true;

    for _ in 0..n {
        input! {
            times: i32,
            x: i32,
            y: i32
        }
        masu.push((times, x, y));
    }
    for j in 1..=n {
        let d = { (masu[j].1 - masu[j - 1].1).abs() + (masu[j].2 - masu[j - 1].2).abs() };
        let sa = masu[j].0 - masu[j - 1].0;
        // println!("d is:{}, sa is{}", d, sa);
        if sa < d {
            yes = false
        }
        if ({ sa - d } % 2) != 0 {
            yes = false
        }
    }
    println!("{}", if yes { "Yes" } else { "No" });
    // for a in masu {
    //     print!("{:?}", a);
    // }
}
