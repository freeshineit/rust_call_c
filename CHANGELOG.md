# 更新日志

本文档记录项目的所有重要变更。

## [未发布]

### 新增
- 完整的项目文档（README.md）
- 三个 FFI 示例子项目：
  - call_c: 动态编译 C 代码
  - call_cpp: 动态编译 C++ 代码
  - call_c_static: 链接静态库
- GitHub Actions CI 工作流
- 贡献指南（CONTRIBUTING.md）
- MIT 许可证
- .gitignore 文件

### 修复
- 修复 Rust 2024 edition 的 `unsafe extern` 语法
- 改进 build.rs 脚本的注释和结构
- 统一代码风格

### 改进
- 为所有子项目添加详细的 README
- 添加静态库编译说明
- 改进代码注释
- 添加常见问题解答

## [0.1.0] - 初始版本

### 新增
- 基础项目结构
- C/C++ FFI 示例
