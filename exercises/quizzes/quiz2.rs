// 本测验涵盖以下章节：
// - 字符串
// - Vecs
// - 移动语义
// - 模块
// - 枚举
//
// 我们来用函数构建一个小机器。输入是一组字符串和命令，这些命令决定对字符串
// 执行什么操作。操作可以是：
// - 将字符串转换为大写
// - 修剪字符串
// - 将 "bar" 追加到字符串末尾指定次数
//
// 具体形式如下：
// - 输入是一个由二元组组成的 Vector，第一个元素是字符串，第二个元素是命令。
// - 输出是一个字符串向量。

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: 按照上面的描述完成函数。
    // pub fn transformer(input: ???) -> ??? { ??? }
}

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    // TODO: 需要导入什么才能让 `transformer` 位于当前作用域？
    // use ???;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
