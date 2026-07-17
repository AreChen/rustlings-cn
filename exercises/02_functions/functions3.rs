fn call_me(num: u8) {
    for i in 0..num {
        println!("响铃！第 {} 次", i + 1);
    }
}

fn main() {
    // TODO: 修复函数调用。
    call_me();
}
