// 1. Fix the error, do not add new lines of code
// 🌟 We cannot use str type directly, but we can use &str instead

pub fn test1() {
    let s: &str = "hello, world";
}

// 🌟🌟 If we want to use str type, we can only use Box. & can be used to convert Box<str> to &str type
pub fn test2() {
    let s: Box<str> = "hello, world".into();
    greetings(&s);
}

fn greetings(s: &str) {
    println!("{}", s);
}

// String 是定义在标准库中的类型，分配在堆上，可以动态的增长。它的底层存储是动态字节数组的方式( Vec<u8> )，但是与字节数组不同，String 是 UTF-8 编码。

fn test3() {
    let mut s = String::from("");
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");
}

fn test4() {
    let s = String::from("hello");
    s.push(',');
    s.push(" world");
    s += "!".to_string();

    println!("{}", s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functions() {
        test1();
        test2();
        test3();
    }
}
