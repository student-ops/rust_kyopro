use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [(i64, i64); n],
    }

    let mut palatability_by_color: HashMap<i64, Vec<i64>> = HashMap::new();

    // 色ごとにビーンズのおいしさを格納する
    for (palatability, color) in a {
        palatability_by_color
            .entry(color)
            .or_insert(Vec::new())
            .push(palatability);
    }

    let mut max_min_palatability = 0;

    // 各色のグループの中で最も不味いビーンズを探索
    for palatabilities in palatability_by_color.values() {
        let min_palatability = palatabilities.iter().min().unwrap();
        max_min_palatability = max_min_palatability.max(*min_palatability);
    }

    println!("{}", max_min_palatability);
}
