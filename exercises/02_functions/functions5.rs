// TODO: 在不修改函数签名的前提下，修复函数体。
fn square(num: i32) -> i32 {
    num * num;
}

fn main() {
    let answer = square(3);
    println!("3 的平方是 {answer}");
}
