// 这与之前的 `from_into` 练习相似。但这次我们将实现 `FromStr` 并返回错误，
// 而不是退回到默认值。此外，实现 `FromStr` 后，可以使用字符串的 `parse` 方法
// 创建实现者类型的对象。你可以在文档中了解更多信息：
// 文档链接：https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// `FromStr` 实现将使用这个错误类型。
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // 字段数量不正确
    BadLen,
    // 姓名字段为空
    NoName,
    // `parse::<u8>()` 包装的错误
    ParseInt(ParseIntError),
}

// TODO: 完成这个 `FromStr` 实现，使其能够从格式为 "Mark,20" 的字符串中
// 解析出 `Person`。
// 注意，需要使用类似 `"4".parse::<u8>()` 的方式将年龄部分解析为 `u8`。
//
// 步骤：
// 1. 根据字符串中的逗号分割给定字符串。
// 2. 如果分割结果少于或多于 2 个元素，返回错误 `ParsePersonError::BadLen`。
// 3. 将分割结果的第一个元素作为姓名。
// 4. 如果姓名为空，返回错误 `ParsePersonError::NoName`。
// 5. 将分割结果的第二个元素解析为 `u8`，作为年龄。
// 6. 如果年龄解析失败，返回错误 `ParsePersonError::ParseInt`。
impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

fn main() {
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!("John,twenty".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(",".parse::<Person>(), Err(NoName | ParseInt(_))));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(NoName | ParseInt(_)),
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
    }
}
