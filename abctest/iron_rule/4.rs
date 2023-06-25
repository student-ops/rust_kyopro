use proconio::input;

fn main(){
    // 下の桁から順番に「2 進法に変換した値」を求める
    input! {
        n:u32
    }

    let mut array: Vec<u32> = Vec::new();
    for x in 0..=9 {
        let wari = 1 << (9 - x); // 2 の (9 - x) 乗
         array.push((n / wari) % 2)
         // 割り算の結果に応じて 0 または 1 を出力
    }
    for i in array {
        print!("{}",i);
    }
    println!()
}