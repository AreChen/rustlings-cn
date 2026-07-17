// 在上一道练习的基础上，我们希望所有线程完成各自的工作。
// 但这一次，需要由生成的线程负责更新共享值：`JobStatus.jobs_done`。

use std::{sync::Arc, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: 如果需要**可变的**共享状态，仅使用 `Arc` 是不够的。
    let status = Arc::new(JobStatus { jobs_done: 0 });

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: 在更新共享值之前，必须先采取一个动作。
            status_shared.jobs_done += 1;
        });
        handles.push(handle);
    }

    // 等待所有任务完成。
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: 打印 `JobStatus.jobs_done` 的值。
    println!("已完成的任务数：{}", todo!());
}
