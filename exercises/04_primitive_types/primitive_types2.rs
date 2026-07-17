// 字符（`char`）

fn main() {
    // 注意这里使用的是 _单引号_，它们不同于你之前看到的双引号。
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // TODO: 仿照前面的例子，在下面声明一个名为 `your_character` 的变量，值为你喜欢的字符。
    // 试试字母、数字（使用单引号）、特殊字符、其他语言中的字符，或者试试表情符号 😉
    // let your_character = '';

    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
