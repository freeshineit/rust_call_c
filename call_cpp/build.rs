fn main() {
    // 当 C++ 源文件改变时重新构建
    println!("cargo:rerun-if-changed=cpplibs/hello.cpp");

    // 使用 cc crate 编译 C++ 代码
    cc::Build::new()
        .cpp(true) // 启用 C++ 模式
        .file("./cpplibs/hello.cpp") // 指定源文件
        .compile("hello"); // 生成 libhello.a
}
