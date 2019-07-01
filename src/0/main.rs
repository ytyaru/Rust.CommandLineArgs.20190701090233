/*
 * Rustでコマンドライン引数を受け取る。
 * CreatedAt: 2019-07-01
 */
fn main() {
//    let args: Vec<String> = std::env::args().collect();
    let args = std::env::args().collect();
    println!("args: {:?}", args);
}

