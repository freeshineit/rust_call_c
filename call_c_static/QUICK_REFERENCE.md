# build.rs 快速参考

## 核心代码（5 行）

```rust
use std::env;

fn main() {
    let lib_path = env::current_dir().unwrap().join("c_static_libs");
    println!("cargo:rustc-link-search=native={}", lib_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=hello");
    println!("cargo:rerun-if-changed=c_static_libs/libhello.a");
}
```

## 三个关键指令

| 指令 | 作用 | 说明 |
|------|------|------|
| `cargo:rustc-link-search=native=<路径>` | 指定库搜索路径 | 告诉链接器在哪里找 `.a` 文件 |
| `cargo:rustc-link-lib=static=<库名>` | 链接静态库 | 链接 `libhello.a`，只需写 `hello` |
| `cargo:rerun-if-changed=<文件>` | 优化构建 | 只在文件改变时重新运行 |

## 命令速查

### 编译静态库
```bash
cd c_static_libs
gcc -c hello.c -o hello.o    # 编译
ar rcs libhello.a hello.o    # 打包
```

### 构建项目
```bash
cargo build -p call_c_static      # 普通构建
cargo build -p call_c_static -vv  # 详细输出
cargo clean -p call_c_static      # 清理
```

### 调试工具
```bash
nm libhello.a                     # 查看符号表
otool -L target/debug/call_c_static  # 查看依赖 (macOS)
ldd target/debug/call_c_static    # 查看依赖 (Linux)
```

## 库命名规则

| 平台 | 静态库文件名 | build.rs 中写法 |
|------|-------------|----------------|
| Linux/macOS | `libhello.a` | `hello` |
| Windows | `hello.lib` | `hello` |

**记住**：只写库名，不要 `lib` 前缀和扩展名！

## 常见错误

| 错误信息 | 原因 | 解决方案 |
|---------|------|---------|
| `library not found for -lhello` | 静态库不存在 | 先编译 `libhello.a` |
| `undefined reference to 'hello'` | 符号未找到 | 检查 C 函数是否正确编译 |
| `building for ... but attempting to link with file built for ...` | 架构不匹配 | 重新编译静态库 |

## 完整流程（3 步）

```bash
# 1. 编译静态库
cd call_c_static/c_static_libs && gcc -c hello.c -o hello.o && ar rcs libhello.a hello.o && cd ../..

# 2. 构建 Rust 项目
cargo build -p call_c_static

# 3. 运行
cargo run -p call_c_static
```

## 更多信息

- 📖 [BUILD_SCRIPT_EXPLAINED.md](BUILD_SCRIPT_EXPLAINED.md) - 详细代码解析
- 🔄 [WORKFLOW.md](WORKFLOW.md) - 完整工作流程
- 📚 [c_static_libs/README.md](c_static_libs/README.md) - 静态库说明
