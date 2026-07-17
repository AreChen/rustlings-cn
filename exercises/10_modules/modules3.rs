// 你可以使用 `use` 关键字将任意模块中的模块路径引入作用域，
// 尤其可以引入标准库中的路径。

// TODO: 将 `std::time` 模块中的 `SystemTime` 和 `UNIX_EPOCH` 引入作用域。
// 如果能用一行完成，还会获得额外的代码风格加分！
// use ???;

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("自 1970-01-01 00:00:00 UTC 起已过去 {} 秒！", n.as_secs()),
        Err(_) => panic!("系统时间早于 UNIX_EPOCH！"),
    }
}
