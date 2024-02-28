#[cfg(test)]
mod tests {
    use std::{
        rc::Rc,
        sync::{Arc, Condvar, Mutex, RwLock},
        thread,
        time::Duration,
    };

    use std::thread::{sleep, spawn};

    //  m.lock明明返回一个锁，怎么就变成我们的num数值了？聪明的读者可能会想到智能指针，没错，
    //  因为Mutex<T>是一个智能指针，准确的说是m.lock()返回一个智能指针MutexGuard<T>:

    // 它实现了Deref特征，会被自动解引用后获得一个引用类型，该引用指向Mutex内部的数据
    // 它还实现了Drop特征，在超出作用域后，自动释放锁，以便其它线程能继续获取锁
    #[test]
    fn test_mutex() {
        let m = Mutex::new(5);

        {
            // 获取锁，然后deref为`m`的引用
            // lock返回的是Result
            let mut num = m.lock().unwrap();
            *num = 6;
            // 锁自动被drop
        }

        println!("m = {:?}", m);
    }
    #[test]
    fn test_mutex_drop() {
        let m: Mutex<i32> = Mutex::new(5);

        let mut num = m.lock().unwrap();
        *num = 6;
        // 锁还没有被 drop 就尝试申请下一个锁，导致主线程阻塞
        drop(num); // 手动 drop num ，可以让 num1 申请到下个锁
        let mut num1 = m.lock().unwrap();
        *num1 = 7;
        // drop(num1); // 手动 drop num1 ，观察打印结果的不同。打印出来的是lock对象。如果加个括号 m num1就会被drop。变成了具体的引用数据。

        println!("m = {:?}", m);
    }

    // #[test]
    // fn test_mutex_multi_thread() {
    //     // 通过`Rc`实现`Mutex`的多所有权
    //     let counter = Rc::new(Mutex::new(0));
    //     let mut handles = vec![];

    //     for _ in 0..10 {
    //         let counter = Rc::clone(&counter);
    //         // 创建子线程，并将`Mutex`的所有权拷贝传入到子线程中
    //         let handle = thread::spawn(move || {
    //             let mut num = counter.lock().unwrap();

    //             *num += 1;
    //         });
    //         handles.push(handle);
    //     }

    //     // 等待所有子线程完成
    //     for handle in handles {
    //         handle.join().unwrap();
    //     }

    //     // 输出最终的计数结果
    //     println!("Result: {}", *counter.lock().unwrap());
    // }

    // 对比上面
    #[test]
    fn test_mutex_multi_thread() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }

    // RwLock在使用上和Mutex区别不大，只有在多个读的情况下不阻塞程序，
    // 其他如读写、写读、写写情况下均会对后获取锁的操作进行阻塞。
    #[test]
    fn test_raw_lock() {
        let lock = RwLock::new(5);

        // 同一时间允许多个读
        {
            let r1 = lock.read().unwrap();
            let r2 = lock.read().unwrap();
            assert_eq!(*r1, 5);
            assert_eq!(*r2, 5);
        } // 读锁在此处被drop

        // 同一时间只允许一个写
        {
            let mut w = lock.write().unwrap();
            *w += 1;
            assert_eq!(*w, 6);

            // 以下代码会阻塞发生死锁，因为读和写不允许同时存在
            // 写锁w直到该语句块结束才被释放，因此下面的读锁依然处于`w`的作用域中
            // let r1 = lock.read();
            // println!("{:?}",r1);
        }
    }

    // 首先简单性上Mutex完胜，因为使用RwLock你得操心几个问题：

    // 读和写不能同时发生，如果使用try_xxx解决，就必须做大量的错误处理和失败重试机制
    // 当读多写少时，写操作可能会因为一直无法获得锁导致连续多次失败(writer starvation)
    // RwLock 其实是操作系统提供的，实现原理要比Mutex复杂的多，因此单就锁的性能而言，比不上原生实现的Mutex
    // 再来简单总结下两者的使用场景：

    // 追求高并发读取时，使用RwLock，因为Mutex一次只允许一个线程去读取
    // 如果要保证写操作的成功性，使用Mutex
    // 不知道哪个合适，统一使用Mutex

    #[test]
    fn test_convar() {
        let flag = Arc::new(Mutex::new(false));
        let cond = Arc::new(Condvar::new());
        let cflag = flag.clone();
        let ccond = cond.clone();

        let hdl = spawn(move || {
            let mut lock = cflag.lock().unwrap();
            let mut counter = 0;

            while counter < 3 {
                while !*lock {
                    // wait方法会接收一个MutexGuard<'a, T>，且它会自动地暂时释放这个锁，使其他线程可以拿到锁并进行数据更新。
                    // 同时当前线程在此处会被阻塞，直到被其他地方notify后，它会将原本的MutexGuard<'a, T>还给我们，即重新获取到了锁，同时唤醒了此线程。
                    lock = ccond.wait(lock).unwrap();
                }

                *lock = false;

                counter += 1;
                println!("inner counter: {}", counter);
            }
        });

        let mut counter = 0;
        loop {
            sleep(Duration::from_millis(1000));
            *flag.lock().unwrap() = true;
            counter += 1;
            if counter > 3 {
                break;
            }
            println!("outside counter: {}", counter);
            cond.notify_one();
        }
        hdl.join().unwrap();
        println!("{:?}", flag);
    }

    // 打印括号 n = 3， Thread-n = 10
    #[test]
    fn test_convar_produce_consume() {
        let cond = Arc::new(Condvar::new());
        let count = Arc::new(Mutex::new(0));
        let ccond = cond.clone();
        let count_clone = Arc::clone(&count);

        for _ in 0..10 {
            let t_consume = spawn(move || {
                let mut lock = count_clone.lock().unwrap();
                loop {
                    if *lock < 3 {
                        lock = ccond.wait(lock).unwrap();
                    }
                    *lock += 1;
                    print!("(");
                    ccond.notify_all();
                }
            });
            let t_produce = spawn(move || {
                let mut lock = count_clone.lock().unwrap();

                loop {
                    if *lock > 0 {
                        lock = ccond.wait(lock).unwrap();
                    }
                    *lock += 1;
                    print!(")");
                    ccond.notify_all();
                }
            });
            t_consume.join();
            t_produce.join();
        }
    }
}
