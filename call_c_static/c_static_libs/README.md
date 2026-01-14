# C 静态库

这个目录包含 C 源代码和编译后的静态库。

## 编译静态库

```bash
# 编译 C 源文件为目标文件
gcc -c hello.c -o hello.o

# 创建静态库
ar rcs libhello.a hello.o

# 清理中间文件（可选）
rm hello.o
```

## 验证静态库

```bash
# 查看静态库包含的符号
nm libhello.a

# 或使用 ar 查看内容
ar -t libhello.a
```

## 注意事项

- 静态库命名规则：`lib<name>.a`（Linux/macOS）或 `<name>.lib`（Windows）
- Rust 中使用 `cargo:rustc-link-lib=static=<name>` 链接时，只需指定 `<name>` 部分
- 确保静态库与目标平台架构匹配
