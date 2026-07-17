// Rust 中的类型转换通过使用 `as` 运算符完成。
// 注意，`as` 运算符不仅用于类型转换，也可以帮助重命名导入项。

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    // TODO: 在相除之前进行类型转换。
    total / values.len()
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
