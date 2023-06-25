// use proconio::input;

fn main() {
    // input! {
    //   s:String
    // }
    // create new string
    let mut sa  =  "hello".to_string();
    // define null string
    let mut sb = String::new();
    for i in 0..sa.len() {
        sb.push(sa.pop().unwrap());
    }
    println!("{}", sb)
}
