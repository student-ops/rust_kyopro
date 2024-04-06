use proconio::input;
use std::f64;

fn calc_dist((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> f64 {
    let dx = (x2 - x1) as f64;
    let dy = (y2 - y1) as f64;
    (dx * dx + dy * dy).sqrt()
}

fn main() {
    input! {
        n: usize,
        a: [(i32, i32); n],
    }

    // println!("{:?}", a[3]);
    // let dist = calc_dist(a[0], a[3]);
    // println!("{}", dist);
    // let dist = calc_dist(a[0], a[1]);
    // println!("{}", dist);
    for i in 0..n {
        let mut max_for_i = 0.0;
        let mut ans: usize = 0;
        for j in 0..n {
            if i == n - j - 1 {
                continue;
            }
            let mut dist = calc_dist(a[i], a[n - 1 - j]);
            if dist >= max_for_i {
                max_for_i = dist;
                ans = n - j;
            }
        }
        println!("{}", ans)
    }
}
