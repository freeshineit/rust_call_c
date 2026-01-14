## Call C from Rust

这个示例展示如何在 Rust 中调用 C 代码。使用 `cc` crate 在构建时自动编译 C 源文件。

### 运行示例

```bash
# 编译并运行
cargo run -p call_c
```

### 工作原理

1. `build.rs` 使用 `cc` crate 编译 `clibs/hello.c`
2. Rust 代码通过 `extern "C"` 声明 C 函数
3. 在 `unsafe` 块中调用 C 函数

