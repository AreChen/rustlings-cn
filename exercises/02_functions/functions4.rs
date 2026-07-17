// 这家商店正在打折：如果价格是偶数，可以减去 10 Rustbucks；
// 如果价格是奇数，则可以减去 3 Rustbucks。
// 现在不用担心函数体本身，我们暂时只关注函数签名。

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: 修复函数签名。
fn sale_price(price: i64) -> {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("折后价格为 {}", sale_price(original_price));
}
