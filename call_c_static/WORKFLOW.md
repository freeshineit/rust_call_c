# call_c_static 工作流程详解

## 整体流程图

```
┌─────────────────────────────────────────────────────────────┐
│                    1. 准备阶段                               │
│  编译 C 代码为静态库                                          │
│                                                              │
│  hello.c  ──gcc──>  hello.o  ──ar──>  libhello.a           │
│  (源文件)          (目标文件)         (静态库)                │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                    2. 构建阶段                               │
│  运行 cargo build                                            │
│                                                              │
│  cargo build -p call_c_static                               │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                    3. 执行 build.rs                          │
│                                                              │
│  ┌────────────────────────────────────────────────┐         │
│  │ 1. 获取当前目录                                 │         │
│  │    current_dir = /path/to/call_c_static       │         │
│  └────────────────────────────────────────────────┘         │
│                      ↓                                       │
│  ┌────────────────────────────────────────────────┐         │
│  │ 2. 构建库路径                                   │         │
│  │    lib_path = current_dir/c_static_libs       │         │
│  └────────────────────────────────────────────────┘         │
│                      ↓                                       │
│  ┌────────────────────────────────────────────────┐         │
│  │ 3. 发送指令给 cargo                             │         │
│  │    cargo:rustc-link-search=native=<lib_path>  │         │
│  │    cargo:rustc-link-lib=static=hello          │         │
│  │    cargo:rerun-if-changed=libhello.a          │         │
│  └────────────────────────────────────────────────┘         │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                    4. 编译 Rust 代码                         │
│                                                              │
│  rustc src/main.rs                                          │
│    -L native=/path/to/c_static_libs                         │
│    -l static=hello                                          │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                    5. 链接阶段                               │
│                                                              │
│  链接器查找并链接 libhello.a                                 │
│                                                              │
│  main.o + libhello.a ──>  call_c_static (可执行文件)        │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│                    6. 运行程序                               │
│                                                              │
│  ./target/debug/call_c_static                               │
│  输出: Hello, build clang static lib !!!!!!!!               │
└─────────────────────────────────────────────────────────────┘
```

## build.rs 执行细节

### 输入
- 当前目录：`/Users/username/project/rust_call_c/call_c_static`
- 静态库文件：`c_static_libs/libhello.a`

### 处理过程

```rust
// 1. 获取当前目录
let current_dir = env::current_dir().unwrap();
// 结果: PathBuf("/Users/username/project/rust_call_c/call_c_static")

// 2. 拼接路径
let lib_path = current_dir.join("c_static_libs");
// 结果: PathBuf("/Users/username/project/rust_call_c/call_c_static/c_static_libs")

// 3. 发送指令
println!("cargo:rustc-link-search=native={}", lib_path.to_str().unwrap());
// 输出: cargo:rustc-link-search=native=/Users/username/project/rust_call_c/call_c_static/c_static_libs

println!("cargo:rustc-link-lib=static=hello");
// 输出: cargo:rustc-link-lib=static=hello

println!("cargo:rerun-if-changed=c_static_libs/libhello.a");
// 输出: cargo:rerun-if-changed=c_static_libs/libhello.a
```

### 输出（发送给 cargo）

```
cargo:rustc-link-search=native=/Users/username/project/rust_call_c/call_c_static/c_static_libs
cargo:rustc-link-lib=static=hello
cargo:rerun-if-changed=c_static_libs/libhello.a
```

## 链接器工作原理

### 1. 链接器接收参数

```bash
-L native=/path/to/c_static_libs  # 库搜索路径
-l static=hello                    # 要链接的库
```

### 2. 链接器查找库文件

在 `/path/to/c_static_libs` 目录中查找：
- Linux/macOS: `libhello.a`
- Windows: `hello.lib`

### 3. 链接器读取符号表

从 `libhello.a` 中提取：
```
符号表:
  hello (函数)
```

### 4. 链接器解析引用

Rust 代码中的 `extern "C" { fn hello(); }` 需要 `hello` 符号

### 5. 链接器合并代码

```
main.o (Rust 编译产物)
  + 
libhello.a (C 静态库)
  =
call_c_static (最终可执行文件)
```

## 文件依赖关系

```
call_c_static/
├── Cargo.toml              # 项目配置
├── build.rs                # 构建脚本 ← 关键文件
├── c_static_libs/
│   ├── hello.c             # C 源文件
│   └── libhello.a          # 静态库 ← build.rs 需要这个文件
└── src/
    └── main.rs             # Rust 主程序 ← 调用 C 函数

依赖关系:
main.rs ──需要──> hello() 函数
                    ↑
                    │
build.rs ──配置链接──> libhello.a ──包含──> hello() 函数
```

## 与 call_c 的对比

### call_c（动态编译）

```
hello.c ──build.rs(cc crate)──> libhello.a ──链接──> 可执行文件
        自动编译                自动链接
```

**优点**：
- 自动化，无需手动编译
- 跨平台
- 源码可见

**缺点**：
- 每次构建都要编译 C 代码
- 需要 C 编译器

### call_c_static（链接静态库）

```
hello.c ──手动编译──> libhello.a ──build.rs配置──> 可执行文件
        gcc + ar              手动链接
```

**优点**：
- 可以使用第三方预编译库
- 不需要每次都编译 C 代码
- 适合大型 C 库

**缺点**：
- 需要手动编译静态库
- 需要管理不同平台的库文件

## 调试技巧

### 1. 查看详细构建过程

```bash
cargo build -p call_c_static -vv
```

输出示例：
```
[call_c_static 0.1.0] cargo:rustc-link-search=native=/path/to/c_static_libs
[call_c_static 0.1.0] cargo:rustc-link-lib=static=hello
[call_c_static 0.1.0] cargo:rerun-if-changed=c_static_libs/libhello.a
```

### 2. 检查静态库内容

```bash
# 查看静态库包含的符号
nm c_static_libs/libhello.a

# 输出示例:
# hello.o:
# 0000000000000000 T _hello
```

### 3. 检查可执行文件依赖

```bash
# macOS
otool -L target/debug/call_c_static

# Linux
ldd target/debug/call_c_static
```

### 4. 强制重新构建

```bash
# 清理并重新构建
cargo clean -p call_c_static
cargo build -p call_c_static -vv
```

## 常见错误及解决方案

### 错误 1: 找不到库文件

```
error: linking with `cc` failed
ld: library not found for -lhello
```

**原因**：静态库不存在或路径错误

**解决**：
```bash
cd c_static_libs
gcc -c hello.c -o hello.o
ar rcs libhello.a hello.o
```

### 错误 2: 符号未定义

```
undefined reference to `hello'
```

**原因**：
- 静态库中没有 `hello` 函数
- 函数名不匹配（C++ name mangling）

**解决**：
- 检查 C 代码是否正确编译
- 确保使用 `extern "C"` 避免 name mangling

### 错误 3: 架构不匹配

```
ld: warning: ignoring file libhello.a, building for macOS-arm64 but attempting to link with file built for macOS-x86_64
```

**原因**：静态库架构与目标架构不匹配

**解决**：重新编译静态库，确保架构匹配
```bash
gcc -arch arm64 -c hello.c -o hello.o
ar rcs libhello.a hello.o
```

## 总结

`build.rs` 的三个核心任务：

1. **指定搜索路径**：告诉链接器在哪里找库文件
2. **指定链接库**：告诉链接器链接哪个库
3. **优化构建**：只在必要时重新构建

这样 Rust 就能成功调用预编译的 C 静态库了！
