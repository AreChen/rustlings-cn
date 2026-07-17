// 假设我们正在编写一个可以用代币购买物品的游戏。每件物品售价 5 个代币，
// 每次购买还会收取 1 个代币的处理费。玩家会输入想购买的物品数量，
// `total_cost` 函数将计算物品的总价。由于玩家输入的是数量字符串，
// 他们可能输入任何内容，而不只是数字！
//
// 现在这个函数完全没有处理错误情况。我们希望这样做：如果对非数字字符串
// 调用 `total_cost`，该函数会返回 `ParseIntError`。此时，我们应立即从当前
// 函数返回该错误，而不是继续做乘法和加法。
//
// 至少有两种正确的实现方式，不过其中一种要短得多！

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // TODO: 按照上面的描述处理错误情况。
    let qty = item_quantity.parse::<i32>();

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}
