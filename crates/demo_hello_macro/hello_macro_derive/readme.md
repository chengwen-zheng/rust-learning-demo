这种形式的过程宏定义是相当通用的，下面来分析下这段代码。

首先有一点，对于绝大多数过程宏而言，这段代码往往只在 impl_hello_macro(&ast) 中的实现有所区别，对于其它部分基本都是一致的，如包的引入、宏函数的签名、语法树构建等。

proc_macro 包是 Rust 自带的，因此无需在 Cargo.toml 中引入依赖，它包含了相关的编译器 API，可以用于读取和操作 Rust 源代码。

由于我们为 hello_macro_derive 函数标记了 #[proc_macro_derive(HelloMacro)]，当用户使用 #[derive(HelloMacro)] 标记了他的类型后，hello_macro_derive 函数就将被调用。这里的秘诀就是特征名 HelloMacro，它就像一座桥梁，将用户的类型和过程宏联系在一起。

syn 将字符串形式的 Rust 代码解析为一个 AST 树的数据结构，该数据结构可以在随后的 impl_hello_macro 函数中进行操作。最后，操作的结果又会被 quote 包转换回 Rust 代码。这些包非常关键，可以帮我们节省大量的精力，否则你需要自己去编写支持代码解析和还原的解析器，这可不是一件简单的任务！
