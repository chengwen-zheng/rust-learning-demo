#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_thread_mpsc() {
        // 创建一个消息通道, 返回一个元组：(发送者，接收者)
        let (tx, rx) = mpsc::channel();

        // 创建线程，并发送消息
        thread::spawn(move || {
            // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
            tx.send(1).unwrap();

            // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
            // tx.send(Some(1)).unwrap()
        });

        // 在主线程中接收子线程发送的消息并输出
        println!("receive {}", rx.recv().unwrap());
    }

    // 除了上述recv方法，还可以使用try_recv尝试接收一次消息，该方法并不会阻塞线程，当通道中没有消息时，
    // 它会立刻返回一个错误：

    #[test]
    fn test_thread_try_recv() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            tx.send(1).unwrap();
        });

        println!("receive {:?}", rx.try_recv());
    }

    #[test]
    fn test_thread_channel_ownership() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            // 若值的类型实现了Copy特征，则直接复制一份该值，然后传输过去，例如之前的i32类型
            // 若值没有实现Copy，则它的所有权会被转移给接收端，在发送端继续使用该值将报错
            let s = 0;
            // let s = String::from("我，飞走咯!");
            tx.send(s).unwrap();
            println!("val is {}", s);
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    #[test]
    fn test_thread_for_received() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }

    //需要所有的发送者都被drop掉后，接收者rx才会收到错误，进而跳出for循环，最终结束主线程
    //这里虽然用了clone但是并不会影响性能，因为它并不在热点代码路径中，仅仅会被执行一次
    //由于两个子线程谁先创建完成是未知的，因此哪条消息先发送也是未知的，最终主线程的输出顺序也不确定
    #[test]
    fn test_thread_multi_sender() {
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();
        thread::spawn(move || {
            tx.send(String::from("hi from raw tx")).unwrap();
        });

        thread::spawn(move || {
            tx1.send(String::from("hi from cloned tx")).unwrap();
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }
}
