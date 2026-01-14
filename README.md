# Rust FFI 示例项目

[![CI](https://github.com/yourusername/rust_call_c/workflows/CI/badge.svg)](https://github.com/yourusername/rust_call_c/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

这个项目展示了 Rust 与 C/C++ 代码互操作的三种常见方式，适合学习 Rust FFI（Foreign Function Interface）的开发者。

## 项目结构

```
rust_call_c/
├── call_c/              # 使用 cc crate 动态编译 C 代码
│   ├── clibs/          # C 源文件
│   ├── build.rs        # 构建脚本
│   └── src/main.rs     # Rust 主程序
├── call_cpp/            # 使用 cc crate 动态编译 C++ 代码
│   ├── cpplibs/        # C++ 源文件
│   ├── build.rs        # 构建脚本
│   └── src/main.rs     # Rust 主程序
└── call_c_static/       # 链接预编译的 C 静态库
    ├── c_static_libs/  # 静态库目录
    ├── build.rs        # 构建脚本
    └── src/main.rs     # Rust 主程序
```

## 快速开始

### 前置要求

- Rust 工具链（1.70+）
- C/C++ 编译器（GCC、Clang 或 MSVC）

### 1. 调用 C 代码

使用 `cc` crate 在构建时自动编译 C 源文件：

```bash
cargo run -p call_c
```

输出：
```
Hello, build clang!!!! 9999000
```

### 2. 调用 C++ 代码

使用 `cc` crate 在构建时自动编译 C++ 源文件：

```bash
cargo run -p call_cpp
```

输出：
```
Hello, world cpp!!!!!
```

### 3. 链接静态库

首先编译 C 代码为静态库：

```bash
cd call_c_static/c_static_libs
gcc -c hello.c -o hello.o
ar rcs libhello.a hello.o
cd ../..
```

然后运行程序：

```bash
cargo run -p call_c_static
```

输出：
```
Hello, build clang static lib !!!!!!!!
```

## 技术要点

### FFI 基础

- **extern 声明**：使用 `unsafe extern "C"` 声明外部函数（Rust 2024 edition）
- **unsafe 块**：调用 FFI 函数必须在 `unsafe` 块中
- **ABI 指定**：`"C"` 表示使用 C 调用约定

### 构建脚本

- **build.rs**：在编译 Rust 代码前执行的构建脚本
- **cc crate**：简化跨平台 C/C++ 代码编译
- **cargo 指令**：通过 `println!` 向 cargo 传递编译指令

### C++ 互操作

- **extern "C"**：在 C++ 中使用 `extern "C"` 避免名称修饰（name mangling）
- **C++ 模式**：在 `cc::Build` 中设置 `.cpp(true)` 启用 C++ 编译

## 示例代码

### Rust 端

```rust
unsafe extern "C" {
    fn hello();
}

fn main() {
    unsafe {
        hello();
    }
}
```

### C 端

```c
#include <stdio.h>

void hello() {
    printf("Hello from C!\n");
}
```

### C++ 端

```cpp
#include <iostream>

extern "C" {
    void hello();
}

void hello() {
    std::cout << "Hello from C++!" << std::endl;
}
```

## 常见问题

### Q: 为什么需要 unsafe？

A: FFI 调用无法保证内存安全，Rust 要求显式标记为 `unsafe`，提醒开发者注意潜在风险。

### Q: 如何传递复杂数据类型？

A: 建议使用 C 兼容的类型（如 `#[repr(C)]` 结构体）或通过指针传递。

### Q: 如何处理字符串？

A: 使用 `std::ffi::CString` 和 `CStr` 在 Rust 字符串和 C 字符串之间转换。

### Q: 支持哪些平台？

A: 理论上支持所有 Rust 支持的平台，但需要相应的 C/C++ 编译器。

## 进阶主题

- 传递结构体和数组
- 回调函数
- 错误处理
- 内存管理
- 线程安全

查看各子项目的 README 了解更多细节。

## 相关资源

- [Rust FFI 官方文档](https://doc.rust-lang.org/nomicon/ffi.html)
- [cc crate 文档](https://docs.rs/cc/)
- [Rust Nomicon](https://doc.rust-lang.org/nomicon/)

## 贡献

欢迎提交 Issue 和 Pull Request！详见 [CONTRIBUTING.md](CONTRIBUTING.md)。

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。