# build.rs 详细说明

## 什么是 build.rs？

`build.rs` 是 Rust 的**构建脚本**，在编译主项目代码之前执行。它可以：
- 编译 C/C++ 代码
- 生成代码
- 配置链接器参数
- 设置环境变量

## 代码逐行解析

### 1. 导入标准库

```rust
use std::env;
```

**作用**：导入 `env` 模块，用于获取环境信息（如当前目录）。

---

### 2. 主函数

```rust
fn main() {
```

**作用**：构建脚本的入口点，cargo 会执行这个函数。

---

### 3. 获取当前目录

```rust
let current_dir = env::current_dir().unwrap();
```

**作用**：
- `env::current_dir()` 获取构建脚本运行时的当前目录
- 返回 `Result<PathBuf>`，使用 `.unwrap()` 获取实际路径
- 这个目录就是 `call_c_static/` 目录

**示例输出**：
```
/Users/username/project/rust_call_c/call_c_static
```

---

### 4. 构建静态库路径

```rust
let lib_path = current_dir.join("c_static_libs");
```

**作用**：
- 使用 `join()` 方法拼接路径
- 生成静态库所在的完整路径

**示例输出**：
```
/Users/username/project/rust_call_c/call_c_static/c_static_libs
```

---

### 5. 告诉链接器在哪里搜索库文件

```rust
println!(
    "cargo:rustc-link-search=native={}",
    lib_path.to_str().unwrap()
);
```

**作用**：
- 通过 `println!` 向 cargo 发送指令
- `cargo:rustc-link-search=native=<path>` 告诉链接器在指定路径搜索库文件
- `native` 表示这是本地系统的库路径
- `.to_str().unwrap()` 将 `PathBuf` 转换为字符串

**等价于命令行**：
```bash
rustc -L native=/path/to/c_static_libs
```

**实际输出到 cargo**：
```
cargo:rustc-link-search=native=/Users/username/project/rust_call_c/call_c_static/c_static_libs
```

---

### 6. 告诉链接器链接哪个库

```rust
println!("cargo:rustc-link-lib=static=hello");
```

**作用**：
- `cargo:rustc-link-lib=static=hello` 告诉链接器链接静态库
- `static` 表示链接静态库（而非动态库）
- `hello` 是库名称

**重要规则**：
- 只需指定库名 `hello`
- **不需要** `lib` 前缀
- **不需要** `.a` 后缀
- 链接器会自动查找 `libhello.a`（Linux/macOS）或 `hello.lib`（Windows）

**等价于命令行**：
```bash
rustc -l static=hello
```

---

### 7. 设置重新构建条件

```rust
println!("cargo:rerun-if-changed=c_static_libs/libhello.a");
```

**作用**：
- 告诉 cargo 只有当 `libhello.a` 文件改变时才重新运行构建脚本
- 这是一个优化，避免不必要的重新构建
- 如果不设置，cargo 会在任何文件改变时都重新运行构建脚本

**工作流程**：
1. 首次构建：运行 build.rs
2. 再次构建：检查 `libhello.a` 是否改变
3. 如果未改变：跳过 build.rs
4. 如果改变了：重新运行 build.rs

---

## 完整工作流程

### 步骤 1：编译静态库

```bash
cd call_c_static/c_static_libs
gcc -c hello.c -o hello.o      # 编译 C 源文件为目标文件
ar rcs libhello.a hello.o      # 打包为静态库
```

生成文件：`libhello.a`

### 步骤 2：运行 cargo build

```bash
cargo build -p call_c_static
```

### 步骤 3：执行 build.rs

1. 获取当前目录：`/path/to/call_c_static`
2. 构建库路径：`/path/to/call_c_static/c_static_libs`
3. 向 cargo 发送指令：
   ```
   cargo:rustc-link-search=native=/path/to/call_c_static/c_static_libs
   cargo:rustc-link-lib=static=hello
   cargo:rerun-if-changed=c_static_libs/libhello.a
   ```

### 步骤 4：编译 Rust 代码

cargo 使用 build.rs 提供的信息编译 `src/main.rs`

### 步骤 5：链接

链接器：
1. 在 `c_static_libs/` 目录搜索库文件
2. 找到 `libhello.a`
3. 将其链接到最终的可执行文件

### 步骤 6：生成可执行文件

最终生成 `target/debug/call_c_static`，包含了静态库中的代码。

---

## cargo 指令参考

### 常用的 cargo 指令

| 指令 | 说明 | 示例 |
|------|------|------|
| `cargo:rustc-link-search=native=<path>` | 添加库搜索路径 | `cargo:rustc-link-search=native=/usr/lib` |
| `cargo:rustc-link-lib=static=<name>` | 链接静态库 | `cargo:rustc-link-lib=static=hello` |
| `cargo:rustc-link-lib=dylib=<name>` | 链接动态库 | `cargo:rustc-link-lib=dylib=ssl` |
| `cargo:rerun-if-changed=<path>` | 文件改变时重新构建 | `cargo:rerun-if-changed=build.rs` |
| `cargo:rustc-flags=<flags>` | 传递编译器标志 | `cargo:rustc-flags=-l dylib=foo` |

---

## 静态库 vs 动态库

### 静态库（Static Library）

```rust
println!("cargo:rustc-link-lib=static=hello");
```

**特点**：
- 文件：`libhello.a`（Linux/macOS）、`hello.lib`（Windows）
- 代码被复制到最终可执行文件中
- 可执行文件更大，但不依赖外部库
- 分发简单，只需一个可执行文件

### 动态库（Dynamic Library）

```rust
println!("cargo:rustc-link-lib=dylib=hello");
```

**特点**：
- 文件：`libhello.so`（Linux）、`libhello.dylib`（macOS）、`hello.dll`（Windows）
- 运行时加载，不包含在可执行文件中
- 可执行文件更小
- 需要在运行时找到动态库文件

---

## 常见问题

### Q1: 为什么需要 build.rs？

A: 因为 Rust 编译器不知道在哪里找静态库，build.rs 告诉它：
- 库文件在哪个目录
- 要链接哪个库
- 什么时候需要重新构建

### Q2: 如果不写 build.rs 会怎样？

A: 链接器会报错：
```
error: linking with `cc` failed
ld: library not found for -lhello
```

### Q3: 可以链接多个库吗？

A: 可以，多次调用 `println!`：
```rust
println!("cargo:rustc-link-lib=static=hello");
println!("cargo:rustc-link-lib=static=world");
println!("cargo:rustc-link-lib=dylib=ssl");
```

### Q4: 如何调试 build.rs？

A: 使用 `cargo build -vv` 查看详细输出：
```bash
cargo build -p call_c_static -vv
```

---

## 对比：call_c vs call_c_static

### call_c 的 build.rs

```rust
// 使用 cc crate 编译 C 源文件
cc::Build::new()
    .file("./clibs/hello.c")
    .compile("hello");
```

**特点**：
- 自动编译 C 源文件
- 自动处理链接
- 跨平台

### call_c_static 的 build.rs

```rust
// 手动指定预编译的静态库
println!("cargo:rustc-link-search=native={}", lib_path);
println!("cargo:rustc-link-lib=static=hello");
```

**特点**：
- 使用预编译的库
- 需要手动编译静态库
- 更灵活，可以使用第三方库

---

## 总结

`build.rs` 的核心作用是**配置链接器**：

1. **告诉链接器在哪里找库**（`rustc-link-search`）
2. **告诉链接器链接哪个库**（`rustc-link-lib`）
3. **优化构建过程**（`rerun-if-changed`）

这样 Rust 代码就能成功调用 C 静态库中的函数了！
