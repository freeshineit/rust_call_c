use std::env;

fn main() {
    // ========== 步骤 1: 获取静态库所在目录 ==========
    //
    // 获取当前工作目录（即 call_c_static/ 目录）
    let current_dir = env::current_dir().unwrap();
    // 拼接静态库目录路径：call_c_static/c_static_libs
    let lib_path = current_dir.join("c_static_libs");

    // ========== 步骤 2: 告诉链接器在哪里搜索库文件 ==========
    //
    // 通过 println! 向 cargo 发送指令
    // cargo:rustc-link-search=native=<路径>
    // - native: 表示这是本地系统的库路径
    // - 等价于命令行: rustc -L native=/path/to/c_static_libs
    println!(
        "cargo:rustc-link-search=native={}",
        lib_path.to_str().unwrap()
    );

    // ========== 步骤 3: 告诉链接器链接哪个静态库 ==========
    //
    // cargo:rustc-link-lib=static=<库名>
    // - static: 表示链接静态库（.a 文件）
    // - hello: 库名称，链接器会自动查找 libhello.a
    // - 注意：只需指定库名 "hello"，不需要 "lib" 前缀和 ".a" 后缀
    // - 等价于命令行: rustc -l static=hello
    println!("cargo:rustc-link-lib=static=hello");
    // ========== 步骤 4: 优化构建过程 ==========
    //
    // cargo:rerun-if-changed=<文件路径>
    // - 只有当指定文件改变时才重新运行此构建脚本
    // - 避免不必要的重新构建，提高编译速度
    println!("cargo:rerun-if-changed=c_static_libs/libhello.a");
    // 提示：如果静态库不存在，需要先编译：
    // cd c_static_libs
    // gcc -c hello.c -o hello.o
    // ar rcs libhello.a hello.o
}
