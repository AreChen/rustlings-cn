// 本练习给出一个名为 `numbers` 的 `u32` 向量，其值从 0 到 99。
// 我们希望让 8 个不同的线程同时使用这组数字。每个线程将计算带有偏移量的
// 每第八个值的总和。
//
// 第一个线程（偏移量 0）计算 0、8、16、…… 的总和。
// 第二个线程（偏移量 1）计算 1、9、17、…… 的总和。
// 第三个线程（偏移量 2）计算 2、10、18、…… 的总和。
// …
// 第八个线程（偏移量 7）计算 7、15、23、…… 的总和。
//
// 每个线程都应拥有指向数字向量的引用计数指针。但 `Rc` 不是线程安全的，
// 因此需要使用 `Arc`。
//
// 不要被线程的生成和合并方式分散注意力。我们会在后面的线程练习中练习这些内容。

// 不要修改下面的几行。
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO: 使用 `Arc` 定义 `shared_numbers`。
    // let shared_numbers = ???;

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO: 使用 `shared_numbers` 定义 `child_numbers`。
        // let child_numbers = ???;

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("偏移量 {offset} 的总和为 {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
