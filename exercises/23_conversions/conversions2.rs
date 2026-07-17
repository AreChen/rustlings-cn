// `From` trait 用于值到值的转换。实现 `From` 后，系统会自动提供 `Into` 的实现。
// 你可以在文档中了解更多信息：
// 文档链接：https://doc.rust-lang.org/std/convert/trait.From.html
//
// 使用不同类型表示度量单位是一种常见做法。
// 这样可以避免意外混淆不同度量单位的值。

struct Celsius(f64);

struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    // TODO: 将摄氏度转换为华氏度。不必担心浮点精度。
    // 公式为：F = C * 1.8 + 32
}

impl From<Fahrenheit> for Celsius {
    // TODO: 将华氏度转换为摄氏度。
}

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: [(f64, f64); 6] = [
        (-50.0, -58.0),
        (0.0, 32.0),
        (20.0, 68.0),
        (100.0, 212.0),
        (400.0, 752.0),
        (1000.0, 1832.0),
    ];

    #[test]
    fn celsius_to_fahrenheit() {
        for (celsius, fahrenheit) in CASES {
            let Fahrenheit(actual) = Celsius(celsius).into();
            assert_eq!(actual.round(), fahrenheit);
        }
    }

    #[test]
    fn fahrenheit_to_celsius() {
        for (celsius, fahrenheit) in CASES {
            let Celsius(actual) = Fahrenheit(fahrenheit).into();
            assert_eq!(actual.round(), celsius);
        }
    }
}
