fn main() {
    // 当 C 源文件改变时重新构建
    println!("cargo:rerun-if-changed=clibs/hello.c");

    // 使用 cc crate 编译 C 代码
    cc::Build::new()
        .file("./clibs/hello.c")
        .shared_flag(false) // 编译为静态库
        .compile("hello"); // 生成 libhello.a
}
