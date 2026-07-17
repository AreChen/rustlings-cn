// 应将对此函数的调用替换为对 `string_slice` 或 `string` 的调用。
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: 下面有许多值，其中一些是 `String`，另一些是 `&str`。
// 你的任务是根据对每个值类型的判断，将 `placeholder(…)` 替换为
// `string_slice(…)` 或 `string(…)`。
fn main() {
    placeholder("blue");

    placeholder("red".to_string());

    placeholder(String::from("hi"));

    placeholder("rust is fun!".to_owned());

    placeholder(format!("Interpolation {}", "Station"));

    // WARNING：这是字节索引，不是字符索引。
    // 可以使用 `s.chars().nth(INDEX)` 进行字符索引。
    placeholder(&String::from("abc")[0..1]);

    placeholder("  hello there ".trim());

    placeholder("Happy Monday!".replace("Mon", "Tues"));

    placeholder("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
