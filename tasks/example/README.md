<h1 style="text-align:center">试题示例</h1>

### [试题一](./src/main.rs)

本题中你需要证明对于Rust的工具链获取和管理有一定理解，在文件中试着使用如下命令
``` shell
$ rustup toolchain list
```
并确认自己的rust版本和工具链

### [试题二](./src/lib_greet)

本题中需要理解rust的基本代码组织

在[`organization.rs`](./src/lib_greet/organization.rs)中创建一个`pub struct`，然后在[`main.rs`](./src/main.rs)中引用它

此外你仍然需要在[`main.rs`](./src/main.rs)创建一个`inline struct`，之后在[`main.rs`](./src/main.rs)的`test`中使用它

比如：
```rust
#[test]
fn test() {
    super::CertainInlineStruct::certain_function();
}
```

### [试题三](./src/lib_str)

本题考察对于rust中`struct` `trait`以及声明宏相关的基础理解。

你需要在[`string_proc.rs`](./src/lib_src/string_proc.rs)中完成整个`SelfString` struct的构建

同时需要一个`self_format!` 达成对`String`的`format!`的模仿。

你最终需要达成的效果如下：
```rust
let var = xxx; // something that implements Display
let s_str: SelfString = self_format!("what is this {}", var)
```

此外需要注意`SelfString`的一些别的基础函数和 trait，比如`new()`，`Deref`
