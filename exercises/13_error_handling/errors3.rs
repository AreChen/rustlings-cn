// 这个程序试图使用上一道练习中已经完成的 `total_cost` 函数，但它并不能正常工作！
// 为什么？我们应该如何修复它？

use std::num::ParseIntError;

// 不要修改这个函数。
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: 修改 `main` 函数的签名和函数体，修复编译器错误。
fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // 不要修改这一行。
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("你买不起这么多！");
    } else {
        tokens -= cost;
        println!("你现在有 {tokens} 个代币。");
    }
}
