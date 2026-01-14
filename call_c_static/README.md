## Link Static C Library

这个示例展示如何在 Rust 中链接预编译的 C 静态库。

### 准备静态库

首先需要编译 C 代码为静态库：

```bash
cd call_c_static/c_static_libs
# 编译 C 源文件为目标文件
gcc -c hello.c -o hello.o
# 创建静态库
ar rcs libhello.a hello.o
```

### 运行示例

```bash
# 编译并运行
cargo run -p call_c_static
```

### 工作原理

1. `build.rs` 指定静态库搜索路径和库名称
2. Rust 代码通过 `extern "C"` 声明 C 函数
3. 链接器在构建时链接静态库 `libhello.a`

