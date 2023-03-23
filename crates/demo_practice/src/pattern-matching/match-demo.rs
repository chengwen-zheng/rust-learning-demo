enum Direction {
    East,
    West,
    North,
    South,
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum MyEnum {
    Foo,
    Bar,
}

// 填空
enum Foo {
    Bar(u8),
}

enum Foo1 {
    Bar,
    Baz,
    Qux(u32),
}

fn show_message(msg: Message) {
    match msg {
        Message::Move { x: a, y: b } => {
            // 这里匹配 Message::Move
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        }
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        }
        __ => println!("no data in these variants"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let msgs = [
            Message::Quit,
            Message::Move { x: 1, y: 3 },
            Message::ChangeColor(255, 255, 0),
        ];

        for msg in msgs {
            show_message(msg)
        }

        let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

        // 使用 `matches` 填空
        for ab in alphabets {
            matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9');
            print!("=>{}", ab);
        }

        let mut count = 0;

        let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
        for e in v {
            if matches!(e, MyEnum::Foo) {
                // 修复错误，只能修改本行代码
                // 修复错误，只能修改本行代码
                count += 1;
            }
        }

        print!("count{}", count);

        let o = Some(7);

        // 移除整个 `match` 语句块，使用 `if let` 替代
        match o {
            Some(i) => {
                println!("This is a really long string and `{:?}`", i);
            }
            _ => {}
        };

        if let Some(i) = o {
            println!("This is a really long string and `{:?}`", i);
        }

        let a = Foo::Bar(1);

        if let Foo::Bar(i) = a {
            println!("foobar 持有的值是: {}", i);
        }

        let b = Foo1::Qux(10);

        // 移除以下代码，使用 `match` 代替
        if let Foo1::Bar = b {
            println!("match foo::bar")
        } else if let Foo1::Baz = b {
            println!("match foo::baz")
        } else {
            println!("match others")
        }

        match b {
            Foo1::Bar => println!("match foo::bar"),
            Foo1::Baz => println!("match foo::baz"),
            __ => println!("match others"),
        }

        let age = Some(30);
        if let Some(age) = age {
            // 创建一个新的变量，该变量与之前的 `age` 变量同名
            assert_eq!(age, 30);
        } // 新的 `age` 变量在这里超出作用域

        match age {
            // `match` 也能实现变量遮蔽
            Some(age) => println!("age 是一个新的变量，它的值是 {}", age),
            _ => (),
        }
    }
}
