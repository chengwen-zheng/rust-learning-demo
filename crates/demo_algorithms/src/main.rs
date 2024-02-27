use std::cmp::max;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let s1 = "abcde";
    let s2 = "ace";
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();

    let m = chars1.len();
    let n = chars2.len();

    let round = 2 * max(m, n) - 1;
    let dp = Arc::new(Mutex::new(vec![vec![0; m + 1]; n + 1]));
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];

    for i in 0..round {
        let dp_clone = Arc::clone(&dp);

        let handle = thread::spawn(move || {
            // todo: 将longest_common_subsequence分配给多个线程执行
            // 1. 计算出本轮能够计算的单元格
            let mut dp = dp_clone.lock().unwrap();

            for j in (0..m) {
                let index = i.checked_sub(j).unwrap_or(usize::MAX);
                println!("j:i({}{})", j, index);
            }
        });
        // 2. 将任务分配给线程执行
        handles.push(handle);
        // 3. 等待线程执行完毕
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
