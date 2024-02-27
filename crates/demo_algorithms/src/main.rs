use std::cmp::{max, min};
use std::sync::{Arc, Mutex};
use std::{char, thread};

fn main() {
    let s1 = "abcde";
    let s2 = "ace";

    let chars1: Arc<Vec<char>> = Arc::new(s1.chars().collect());
    let chars2: Arc<Vec<char>> = Arc::new(s2.chars().collect());

    let m = chars1.len();
    let n = chars2.len();

    let round = m + n;
    let dp = Arc::new(Mutex::new(vec![vec![0; m + 1]; n + 1]));

    for i in 0..round {
        let dp_clone: Arc<Mutex<Vec<Vec<i32>>>> = Arc::clone(&dp);
        let mut handles: Vec<thread::JoinHandle<()>> = vec![];
        print!("==============={}: ", i);
        let chars1_clone: Arc<Vec<char>> = chars1.clone();
        let chars2_clone = chars2.clone();
        let handle = thread::spawn(move || {
            // 1. 计算出本轮能够计算的单元格
            let mut dp = dp_clone.lock().unwrap();
            for j in (0..min(i + 1, m)).rev() {
                let index: usize = i - j;

                println!("j:index({}{})", j, index);
            }
        });
        // 2. 将任务分配给线程执行
        handles.push(handle);
        // 3. 等待线程执行完毕
        for handle in handles {
            handle.join().unwrap();
        }
    }

    let dp = dp.lock().unwrap();
    println!("{:?}", dp[n][m]);
    println!("{:?}", dp);
}
