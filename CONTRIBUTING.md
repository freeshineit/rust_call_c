# 贡献指南

感谢你对本项目的关注！

## 开发环境

### 必需工具

- Rust 工具链（推荐使用 rustup）
- C/C++ 编译器：
  - Linux: GCC 或 Clang
  - macOS: Xcode Command Line Tools
  - Windows: MSVC 或 MinGW

### 安装依赖

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# macOS 安装 C/C++ 编译器
xcode-select --install

# Linux (Ubuntu/Debian)
sudo apt-get install build-essential

# Linux (Fedora)
sudo dnf install gcc gcc-c++
```

## 构建项目

```bash
# 构建所有子项目
cargo build --workspace

# 构建特定子项目
cargo build -p call_c
cargo build -p call_cpp
cargo build -p call_c_static
```

## 运行测试

```bash
# 运行所有测试
cargo test --workspace

# 运行特定子项目的测试
cargo test -p call_c
```

## 代码风格

```bash
# 格式化代码
cargo fmt --all

# 检查代码
cargo clippy --workspace -- -D warnings
```

## 添加新示例

1. 在项目根目录创建新的子项目目录
2. 在 `Cargo.toml` 的 `members` 中添加新项目
3. 编写示例代码和文档
4. 更新主 README.md

## 提交规范

- 使用清晰的提交信息
- 每个提交应该是一个逻辑单元
- 确保代码通过 `cargo fmt` 和 `cargo clippy` 检查

## 问题反馈

如果你发现 bug 或有功能建议，请创建 issue 并提供：

- 问题描述
- 复现步骤
- 预期行为
- 实际行为
- 环境信息（操作系统、Rust 版本等）
