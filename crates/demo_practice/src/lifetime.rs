#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        /* 为 `i` 和 `borrow2` 标注合适的生命周期范围 */

        // `i` 拥有最长的生命周期，因为它的作用域完整的包含了 `borrow1` 和 `borrow2` 。
        // 而 `borrow1` 和 `borrow2` 的生命周期并无关联，因为它们的作用域没有重叠
        let i = 3; // `i` lifetime start
        {
            let borrow1 = &i; // `borrow1` 生命周期开始. ──┐
                              //                                                │
            println!("borrow1: {}", borrow1); //              │
        } // `borrow1` 生命周期结束. ──────────────────────────────────┘
        {
            let borrow2 = &i; // `borrow2` lifetime start

            println!("borrow2: {}", borrow2); // `borrow2` lifetime end
        }
        // `i` lifetime end
    }

    /* 像上面的示例一样，为 `r` 和 `x` 标准生命周期，然后从生命周期的角度. */
    #[test]
    fn test1() {
        {
            let r; // ---------+-- 'a
                   //          |
            {
                //          |
                let x = 5; // -+-- 'b  |
                r = &x; //  |       |
                println!("r: {}", r);
            } // -+       |
              //          |
              // println!("r: {}", r); //          |
        } // ---------+  引用的生命周期'a比被引用对象的生命周期'b长，因此编译错误
    }

    #[test]
    fn test2() {
        /* 添加合适的生命周期标注，让下面的代码工作 */
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        longest(&"43", &"45");
    }

    #[test]
    fn test3() {
        /* 使用三种方法修复下面的错误  */
        // fn invalid_output<'a>() -> &'a String {
        //     &String::from("foo")
        // }

        fn invalid_output1<'a>() -> String {
            String::from("foo")
        }

        fn invalid_output2<'a>() -> &'a str {
            "foo"
        }

        fn invalid_output3<'a>() -> &'static str {
            "foo"
        }

        invalid_output1();
        invalid_output2();
        invalid_output3();
    }
    #[test]
    fn test4() {
        // `print_refs` 有两个引用参数，它们的生命周期 `'a` 和 `'b` 至少得跟函数活得一样久
        fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
            println!("x is {} and y is {}", x, y);
        }

        /* 让下面的代码工作 */
        fn failed_borrow<'a>() {
            let _x = 12;

            // let y: &'a i32 = &_x; ERROR: `_x` 活得不够久does not live long enough
            let y: &i32 = &_x;

            // 在函数内使用 `'a` 将会报错，原因是 `&_x` 的生命周期显然比 `'a` 要小
            // 你不能将一个小的生命周期强转成大的
        }

        let (four, nine) = (4, 9);

        print_refs(&four, &nine);
        // 这里，four 和 nice 的生命周期必须要比函数 print_refs 长

        failed_borrow();
        // `failed_borrow`  没有传入任何引用去限制生命周期 `'a`，因此，此时的 `'a` 生命周期是没有任何限制的，它默认是 `'static`
    }

    #[test]
    fn test5() {
        /* 增加合适的生命周期标准，让代码工作 */

        // `i32` 的引用必须比 `Borrowed` 活得更久
        #[derive(Debug)]
        struct Borrowed<'a>(&'a i32);

        // 类似的，下面两个引用也必须比结构体 `NamedBorrowed` 活得更久
        #[derive(Debug)]
        struct NamedBorrowed<'a> {
            x: &'a i32,
            y: &'a i32,
        }

        #[derive(Debug)]
        enum Either<'a> {
            Num(i32),
            Ref(&'a i32),
        }

        let x = 18;
        let y = 15;

        let single = Borrowed(&x);
        let double = NamedBorrowed { x: &x, y: &y };
        let reference = Either::Ref(&x);
        let number = Either::Num(y);

        println!("x is borrowed in {:?}", single);
        println!("x and y are borrowed in {:?}", double);
        println!("x is borrowed in {:?}", reference);
        println!("y is *not* borrowed in {:?}", number);
    }

    #[test]
    fn test6() {
        /* 让代码工作 */

        #[derive(Debug)]
        struct NoCopyType {}

        #[derive(Debug)]
        struct Example<'a, 'b> {
            a: &'a u32,
            b: &'b NoCopyType,
        }

        let var_a = 35;
        let example: Example;
        // 把&var_b的ownership 交出来
        {
            let var_b = &NoCopyType {};

            /* 修复错误 */
            example = Example {
                a: &var_a,
                b: &var_b,
            };
        }

        println!("(Success!) {:?}", example);

        // 把括号去掉。相当于增加了&var_b某个生命周期

        // let var_b = &NoCopyType {};

        // /* 修复错误 */
        // example = Example {
        //     a: &var_a,
        //     b: &var_b,
        // };
        // println!("(Success!) {:?}", example);
    }

    #[test]
    fn test7() {
        #[derive(Debug)]
        struct NoCopyType {}

        #[derive(Debug)]
        #[allow(dead_code)]
        struct Example<'a, 'b> {
            a: &'a u32,
            b: &'b NoCopyType,
        }

        /* 修复函数的签名 */
        fn fix_me<'a>(foo: &'a Example) -> &'a NoCopyType {
            foo.b
        }

        let no_copy = NoCopyType {};
        let example = Example { a: &1, b: &no_copy };
        fix_me(&example);
        println!("Success!")
    }
    #[test]
    fn test8() {
        /* 添加合适的生命周期让下面代码工作 */
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        impl<'a> ImportantExcerpt<'a> {
            fn level(&'a self) -> i32 {
                3
            }
        }
    }

    #[test]
    fn test10() {
        /* 移除所有可以消除的生命周期标注 */
        fn npu(x: i32) {
            println!("`annotated_input`: {}", x);
        }

        fn pass(x: i32) -> i32 {
            x
        }

        fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
            x
        }

        struct Owner(i32);

        impl Owner {
            fn add_one(&mut self) {
                self.0 += 1;
            }
            fn print(&self) {
                println!("`print`: {}", self.0);
            }
        }

        struct Person<'a> {
            age: u8,
            name: &'a str,
        }

        enum Either<'a> {
            Num(i32),
            Ref(&'a i32),
        }
    }
}
