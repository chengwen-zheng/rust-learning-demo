#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        // 填空并修复错误
        use std::num::ParseIntError;

        fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
            let n1 = n1_str.parse::<i32>();
            let n2 = n2_str.parse::<i32>();
            Ok(n1.unwrap() * n2.unwrap())
        }

        let result = multiply("10", "2");
        assert_eq!(result, Ok(20));

        let result = multiply("4", "2");
        assert_eq!(result.unwrap(), 8);

        println!("Success!")
    }
    #[test]
    fn test1() {
        use std::num::ParseIntError;

        // 使用 `?` 来实现 multiply
        // 不要使用 unwrap !
        fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
            let n1 = n1_str.parse::<i32>()?;
            let n2 = n2_str.parse::<i32>()?;
            Ok(n1 * n2)
        }

        assert_eq!(multiply("3", "4").unwrap(), 12);
        println!("Success!")
    }

    #[test]
    fn test2() {
        use std::fs::File;
        use std::io::{self, Read};

        fn read_file1() -> Result<String, io::Error> {
            let f = File::open("hello.txt");
            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut s = String::new();
            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }

        // 填空
        // 不要修改其它代码
        fn read_file2() -> Result<String, io::Error> {
            let mut s = String::new();

            File::open("hello.txt")?.read_to_string(&mut s)?;

            Ok(s)
        }

        assert_eq!(
            read_file1().unwrap_err().to_string(),
            read_file2().unwrap_err().to_string()
        );
        println!("Success!")
    }

    #[test]
    fn test3() {
        use std::num::ParseIntError;

        // 使用两种方式填空: map, and then
        fn add_two1(n_str: &str) -> Result<i32, ParseIntError> {
            n_str.parse::<i32>().map(|i| i * 2)
        }

        fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
            n_str.parse::<i32>().and_then(|num| Ok(num * 2))
        }

        assert_eq!(add_two1("6").unwrap(), 12);
        assert_eq!(add_two("6").unwrap(), 12);

        println!("Success!")
    }
    #[test]
    fn test4() {
        use std::num::ParseIntError;

        // 使用 Result 重写后，我们使用模式匹配的方式来处理，而无需使用 `unwrap`
        // 但是这种写法实在过于啰嗦..
        fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
            match n1_str.parse::<i32>() {
                Ok(n1) => match n2_str.parse::<i32>() {
                    Ok(n2) => Ok(n1 * n2),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            }
        }

        // 重写上面的 `multiply` ，让它尽量简介
        // 提示：使用 `and_then` 和 `map`
        fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
            // 实现...
            let n2 = n2_str.parse::<i32>()?;
            n1_str.parse::<i32>().map(|i| i * n2)
        }

        fn multiply2(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
            // 实现...
            let n2 = n2_str.parse::<i32>()?;
            n1_str.parse::<i32>().and_then(|i| Ok(i * n2))
        }

        fn print(result: Result<i32, ParseIntError>) {
            match result {
                Ok(n) => println!("n is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        let twenty = multiply1("10", "2");
        print(twenty);

        let twenty2 = multiply2("10", "2");
        print(twenty2);

        // 下面的调用会提供更有帮助的错误信息
        let tt = multiply("t", "2");
        print(tt);

        println!("Success!")
    }

    #[test]
    fn test5() {
        use std::num::ParseIntError;
        // 填空 type alias Type Definition https://doc.rust-lang.org/std/io/type.Result.html
        type Res<T> = Result<T, ParseIntError>;

        // 使用上面的别名来引用原来的 `Result` 类型
        fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
            first_number_str.parse::<i32>().and_then(|first_number| {
                second_number_str
                    .parse::<i32>()
                    .map(|second_number| first_number * second_number)
            })
        }

        // 同样, 这里也使用了类型别名来简化代码
        fn print(result: Res<i32>) {
            match result {
                Ok(n) => println!("n is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        print(multiply("10", "2"));
        print(multiply("t", "2"));

        println!("Success!")
    }
}
