// 本练习是 `errors4` 的改编版。它使用了一些课程后面才会讲到的概念，
// 例如 `Box` 和 `From` trait。现在不必详细理解它们，但如果愿意可以提前阅读。
// 目前可以把 `Box<dyn ???>` 类型理解为“我想要任何实现了 ??? 的类型”。
//
// 简单来说，这里的 Box 用例是：你想拥有某个值，但只关心它属于实现了特定
// trait 的类型。为此，将 `Box` 声明为 `Box<dyn Trait>`，其中 `Trait` 是编译器
// 在该上下文中检查值是否实现的 trait。本练习中的上下文是 `Result` 可能返回的错误。

use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// 这是让 `CreationError` 能够实现 `Error` 所必需的。
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// TODO: 添加正确的返回类型 `Result<(), Box<dyn ???>>`。可以用什么来描述这两种错误？
// 是否存在一个由两种错误共同实现的 trait？
fn main() {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("输出={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}
