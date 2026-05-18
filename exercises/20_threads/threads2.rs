// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{sync::{Arc, Mutex}, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

// Arc + Mutex组合
// Arc只提供多线程读的功能 Mutex提供锁的功能 对数据进行了一次锁的封装
// 使用先上锁 然后解包就可以获得ref了

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    let status = Arc::new(Mutex::from(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        // 这里还是不把所有权拿出去 后面status还要拿来打印
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: You must take an action before you update a shared value.
            // 作用域不同 status_shared所有权转移 反正不用后面的了 所以给它转
            // 当然这里引用也是可以的
            status_shared.lock().unwrap().jobs_done += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Print the value of `JobStatus.jobs_done`.
    println!("Jobs done: {}", status.lock().unwrap().jobs_done);
}
