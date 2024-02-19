use demo_hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use hello_macro_derive::show_streams;

#[derive(HelloMacro)]
struct Sunfei;

#[derive(HelloMacro)]
struct Sunface;


// 以下的输出在编译阶段才会打印。可以把target文件夹删掉，然后重新编译，就会看到输出。
// Attribute macros - #[CustomAttribute]
// Example: Basic function
#[show_streams]
fn invoke1() {}
// out: attr: ""
// out: item: "fn invoke1() { }"

// Example: Attribute with input
#[show_streams(bar)]
fn invoke2() {}
// out: attr: "bar"
// out: item: "fn invoke2() {}"·

// Example: Multiple tokens in the input
#[show_streams(multiple => tokens)]
fn invoke3() {}
// out: attr: "multiple => tokens"
// out: item: "fn invoke3() {}"

// Example:
#[show_streams { delimiters }]
fn invoke4() {}
// out: attr: "delimiters"
// out: item: "fn invoke4() {}"

fn main() {
    Sunfei::hello_macro();
    Sunface::hello_macro();

    invoke1();
    invoke2();
    invoke3();
    invoke4();
}
