use std::thread;

struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} is  eating.", self.name);
        thread::sleep(std::time::Duration::from_millis(1000));
        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Judith Butler"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Michel Foucault"),
    ];

    // 在Rust中，闭包可以捕获其环境中的变量。默认情况下，闭包以不可变方式借用（borrow）环境中的变量。
    // 但是，当闭包需要获取环境中的变量所有权时，需要使用move关键字来显式标记。

    // 在给thread::spawn函数传递闭包作为参数时，闭包会捕获它使用的变量。默认情况下，闭包以不可变方式借用这些变量，
    // 因此在并发执行时，闭包无法获取对变量的所有权，这可能导致编译错误或无法正确运行。

    // 为了解决这个问题，需要在闭包前面使用move关键字进行标记。move关键字告诉编译器闭包需要获取环境中变量的所有权，
    // 而不是以借用的方式使用它们。这样，闭包就可以在独立的线程中拥有对这些变量的所有权，而不会发生数据竞争或生命周期问题。

    // 在给定的代码中，map方法中的闭包使用了变量p，它是每个哲学家的实例。由于闭包需要在新线程中执行，
    // 而p变量在当前线程的作用域中，因此使用move关键字将p的所有权转移给闭包，以便在新线程中使用。

    // 通过使用move关键字，闭包可以在独立的线程中获取对p的所有权，并在并发执行时正确地访问和修改它，而不会受到其他线程的干扰。

    // 需要注意的是，使用move关键字会导致闭包获取变量的所有权，这可能改变变量的生命周期和所有权关系。
    // 因此，在使用move关键字时需要注意潜在的副作用和所有权转移。
    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            thread::spawn(move || {
                p.eat();
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
