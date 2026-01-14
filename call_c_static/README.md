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
2. Rust 代码通过 `unsafe extern "C"` 声明 C 函数
3. 链接器在构建时链接静态库 `libhello.a`

---

## 📚 详细文档

### 🚀 快速入门
- **[QUICK_REFERENCE.md](QUICK_REFERENCE.md)** - 快速参考卡片，5 分钟上手

### 📖 深入理解
- **[BUILD_SCRIPT_EXPLAINED.md](BUILD_SCRIPT_EXPLAINED.md)** - build.rs 代码逐行详解
- **[VISUAL_GUIDE.md](VISUAL_GUIDE.md)** - 可视化图解指南
- **[WORKFLOW.md](WORKFLOW.md)** - 完整工作流程和调试技巧

### 📁 其他资源
- **[c_static_libs/README.md](c_static_libs/README.md)** - 静态库编译说明

---

## 🎯 学习路径

1. **初学者**：先看 [QUICK_REFERENCE.md](QUICK_REFERENCE.md)，快速了解核心概念
2. **进阶者**：阅读 [VISUAL_GUIDE.md](VISUAL_GUIDE.md)，理解整个流程
3. **深入研究**：查看 [BUILD_SCRIPT_EXPLAINED.md](BUILD_SCRIPT_EXPLAINED.md)，掌握每行代码
4. **遇到问题**：参考 [WORKFLOW.md](WORKFLOW.md) 的调试技巧

