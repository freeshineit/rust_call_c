use std::env;

fn main() {
    println!("cargo:rustc-link-search=native={}/c_static_libs", env::current_dir().unwrap().to_str().unwrap());
    // 链接静态库, 链接静态库libhello.a
    // libhello.a 文件在c_static_libs目录下
    println!("cargo:rustc-link-lib=static=hello"); 
}