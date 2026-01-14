use std::env;

fn main() {
    // 获取当前目录并指定静态库搜索路径
    let current_dir = env::current_dir().unwrap();
    let lib_path = current_dir.join("c_static_libs");

    println!(
        "cargo:rustc-link-search=native={}",
        lib_path.to_str().unwrap()
    );

    // 链接静态库 libhello.a
    // 注意：只需指定库名 "hello"，不需要 "lib" 前缀和 ".a" 后缀
    println!("cargo:rustc-link-lib=static=hello");

    // 当静态库文件改变时重新构建
    println!("cargo:rerun-if-changed=c_static_libs/libhello.a");
}
